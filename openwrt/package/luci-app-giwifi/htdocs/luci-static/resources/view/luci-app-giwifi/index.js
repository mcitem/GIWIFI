'use strict';
'require view';
'require ui';
'require fs';
'require form';

const DEFAULT_BASE = 'http://100.100.9.2';
const DEFAULT_KEY = '1234567887654321';
const DEFAULT_UA = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0';
const DEFAULT_USERNAME = 'username';
const DEFAULT_PASSWORD = 'password';
const DEFAULT_FORCE = false;


const CRON_COMMAND = '/etc/luci-app-giwifi/task.sh';

/**
 * @typedef {object} This
 * @property {string} crontabContent
 * @property {object} conf
 * @property {form.JSONMap} m
 */

/**
 * 判断 crontab 内容中是否存在某个已启用的条目（且未被注释）
 * @param {string} crontabContent - 完整的 crontab 文件内容
 * @param {string} command - 要检查的命令（需与 crontab 中的命令部分完全匹配，包括参数）
 * @returns {boolean} - 存在且未被注释则返回 true，否则 false
 */
function isCronJobEnabled(crontabContent, command) {
    const lines = crontabContent.replace(/\r\n/g, '\n').split('\n');

    const cronPattern = /^\s*(?:\S+\s+){5}(.*)/;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        const trimmedStart = line.trimStart ? line.trimStart() : line.replace(/^\s+/, '');

        if (!trimmedStart || trimmedStart.startsWith('#')) {
            continue;
        }

        const match = cronPattern.exec(line);
        if (match) {
            const cmdPart = match[1].trim();
            if (cmdPart === command) {
                return true;
            }
        }
    }
    return false;
}

/**
 * 从一行中提取命令及注释状态
 */
function extractCommandFromLine(line) {
    const leadingSpaces = line.match(/^\s*/)[0];
    let contentAfter = line.slice(leadingSpaces.length);
    let isCommented = false;

    if (contentAfter.startsWith('#')) {
        isCommented = true;
        contentAfter = contentAfter.replace(/^#\s?/, '');
    }
    const cronPattern = /^\s*(?:\S+\s+){5}(.*)/;
    const match = cronPattern.exec(contentAfter);
    if (match) {
        const command = match[1].trim();
        return { command, leadingSpaces, isCommented, originalLine: line };
    }
    return null;
}

/**
 * 切换 crontab 中指定命令的启用/禁用状态
 * @param {string} crontabContent - 原始 crontab 内容
 * @param {string} targetCommand - 要切换的命令（需精确匹配，含参数）
 * @param {string} [defaultCronExpr = '0 4 * * *'] - 当条目不存在时新增使用的 cron 时间表达式
 * @returns {string} - 修改后的 crontab 内容
 */
function toggleCronJob(crontabContent, targetCommand, defaultCronExpr = '0 4 * * *') {
    if (typeof crontabContent !== 'string') {
        crontabContent = '';
    }
    const lines = crontabContent.replace(/\r\n/g, '\n').split('\n');

    let toggled = false;
    const newLines = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        if (toggled) {
            newLines.push(line);
            continue;
        }

        const parsed = extractCommandFromLine(line);
        if (parsed && parsed.command === targetCommand) {
            toggled = true;
            // 切换注释状态
            if (parsed.isCommented) {
                // 取消注释
                const uncommented = parsed.originalLine.replace(/^(\s*)#\s?/, '$1');
                newLines.push(uncommented);
            } else {
                // 添加注释
                newLines.push(parsed.leadingSpaces + '# ' + parsed.originalLine.slice(parsed.leadingSpaces.length));
            }
        } else {
            newLines.push(line);
        }
    }

    if (!toggled) {
        newLines.push(`${defaultCronExpr} ${targetCommand}`);

        if (newLines.length && newLines[newLines.length - 1] !== '') {
            newLines.push('');
        }

        return newLines.join('\n');
    }

    return newLines.join('\n');
}

const isReadonlyView = !L.hasViewPermission() || null;

return view.extend({
    load: async function () {
        const crontabContent = await L.resolveDefault(fs.read('/etc/crontabs/root'), '');
        const defaultConf = {
            login: { force: DEFAULT_FORCE, username: DEFAULT_USERNAME, password: DEFAULT_PASSWORD },
            client: {
                base: DEFAULT_BASE,
                key: DEFAULT_KEY,
                ua: DEFAULT_UA
            }
        };
        const conf = JSON.parse(await L.resolveDefault(fs.read('/etc/config/luci-app-giwifi.json'), JSON.stringify(defaultConf)));
        return { crontabContent, conf };
    },
    handleSaveApply: null,
    handleReset: null,

    handleLogin: async function () {
        const self = /** @type {This} */ (this);
        await self.m.save();

        let params = ['login', '-u', self.conf?.login?.username || '', '-p', self.conf?.login?.password || '', '--force'];

        const logFile = '/tmp/giwifi-login.log';
        const pidFile = '/tmp/giwifi-login.pid';

        await fs.write(logFile, '');

        const cmdArgs = params.join(' ');
        fs.exec('/bin/sh', ['-c', `/usr/bin/giwifi ${cmdArgs} > ${logFile} 2>&1 & echo $! > ${pidFile}`]);

        const statusText = E('p', { 'class': 'spinning' }, _('正在执行...'));

        const outputEl = E('pre', {
            'style': 'background: #000; color: #fff; padding: 10px; border-radius: 4px; margin-top: 10px; min-height: 100px; max-height: 400px; overflow-y: auto; font-family: monospace; white-space: pre-wrap;'
        }, '');

        const closeBtn = E('button', {
            'class': 'btn cbi-button-action',
            'style': 'display: none; margin-top: 10px;',
            'click': ui.hideModal
        }, [_('关闭')]);

        const saveBtn = E('button', {
            'class': 'btn cbi-button-action important',
            'style': 'display: none; margin-top: 10px;',
            'click': ui.createHandlerFn(this, 'handleSave')
        }, [_('保存配置')]);

        const modalBody = E('div', {}, [
            statusText,
            outputEl,
            closeBtn,
            saveBtn
        ]);

        ui.showModal(_('登录测试'), modalBody);

        let lastLength = 0;
        let completed = false;
        let pollCount = 0;
        const maxPollCount = 600;

        const pollInterval = setInterval(async () => {
            pollCount++;
            try {
                const content = await L.resolveDefault(fs.read(logFile), '');
                if (content.length > lastLength) {
                    outputEl.textContent = content;
                    lastLength = content.length;
                    outputEl.scrollTop = outputEl.scrollHeight;
                }

                const pidContent = await L.resolveDefault(fs.read(pidFile), '');
                const pid = pidContent.trim();
                if (!pid) {
                    completed = true;
                } else {
                    try {
                        const procResult = await fs.stat('/proc/' + pid);
                        if (!procResult || procResult.type !== 'directory') {
                            completed = true;
                        }
                    } catch (e) {
                        completed = true;
                    }
                }

                if (completed || pollCount >= maxPollCount) {
                    clearInterval(pollInterval);
                    await fs.write(logFile, '');
                    await fs.write(pidFile, '');
                    statusText.classList.remove('spinning');
                    statusText.textContent = completed ? _('执行完成') : _('执行超时');
                    closeBtn.style.display = 'inline-block';
                    if (!completed) {
                        outputEl.textContent += '\n\n执行超时';
                    }
                }
            } catch (e) {
                clearInterval(pollInterval);
                statusText.classList.remove('spinning');
                statusText.textContent = _('执行出错');
                closeBtn.style.display = 'inline-block';
                outputEl.textContent += '\n\n读取输出时发生错误：' + e.message;
            }
        }, 500);

    },


    handleSave: async function () {
        const self = /** @type {This} */ (this);

        await self.m.save();
        if (self.conf?.login) {
            self.conf.login.force = self.conf.login.force === true || self.conf.login.force === 'true' || self.conf.login.force === 1 || self.conf.login.force === '1';
        }

        await fs.write('/etc/config/luci-app-giwifi.json', JSON.stringify({
            login: {
                username: self.conf?.login?.username || DEFAULT_USERNAME,
                password: self.conf?.login?.password || DEFAULT_PASSWORD,
                force: self.conf?.login?.force || DEFAULT_FORCE
            },
            client: {
                base: self.conf?.client?.base || DEFAULT_BASE,
                key: self.conf?.client?.key || DEFAULT_KEY,
                ua: self.conf?.client?.ua || DEFAULT_UA
            }
        }, null, 2));

        // ui.changes.apply();
        location.reload();
    },
    handleToggle: function () {
        const self = /** @type {This} */ (this);
        console.log('Toggling cron job...');
        const result = toggleCronJob(self.crontabContent, CRON_COMMAND);
        fs.write('/etc/crontabs/root', result).then(() => {
            // ui.changes.apply();
            location.reload();
        });
    },
    /**
     * @param {{ crontabContent: string, conf: object }} param0
     */
    render: async function ({ crontabContent, conf }) {
        const self = /** @type {This} */ (this);
        self.crontabContent = crontabContent;
        self.conf = conf;


        const enabled = isCronJobEnabled(crontabContent, CRON_COMMAND);

        console.log('Rendering view with conf:', conf);

        /** @type {form.JSONMap} */ let m;
        /**  @type {form.TypedSection} */ let s;
        /**  @type {form.AbstractValue} */ let o;

        m = new form.JSONMap(
            conf,
        );

        s = m.section(form.TypedSection, 'login', _('登录参数'));

        o = s.option(form.Value, 'username', _('用户名'));
        o.rmempty = false;
        o.placeholder = _('请输入用户名');

        o = s.option(form.Value, 'password', _('密码'));
        o.rmempty = false;
        o.password = true;
        o.placeholder = _('请输入密码');

        o = s.option(form.Flag, 'force', _('强制重新登录'));
        o.rmempty = false;
        o.enabled = 'true';
        o.disabled = 'false';
        o.default = 'false';

        s = m.section(form.TypedSection, 'client', _('客户端设置'));

        o = s.option(form.Value, 'base', _('Base URL'));
        o = s.option(form.Value, 'key', _('Key'));
        o = s.option(form.Value, 'ua', _('User-Agent'));

        self.m = m;
        const formView = await m.render();


        return E([
            E('h2', 'luci-app-giwifi'),
            E('p',
                {
                    'style': 'margin-bottom: 10px;'
                },
                [
                    E('strong', _('计划任务：')),
                    E('span', {}, enabled ? _('Enabled') : _('Disabled'))
                ]
            ),
            E('p', {}, E('button', {
                'class': 'btn cbi-button-action',
                'disabled': isReadonlyView,
                'click': ui.createHandlerFn(this, 'handleToggle')
            }, enabled ? _('Disable') : _('Enable'))),
            formView,
            E('div', { 'class': 'cbi-section-actions', 'style': 'margin-top: 10px;' }, [
                E('button', {
                    'class': 'btn cbi-button-action important',
                    'disabled': isReadonlyView,
                    'click': ui.createHandlerFn(this, 'handleLogin')
                }, _('运行登录测试(断网警告)')),
            ]),
        ]);
    }
});
