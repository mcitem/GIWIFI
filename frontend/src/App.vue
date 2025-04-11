<script setup lang="ts">
import { GalleryVerticalEnd } from "lucide-vue-next";
import { Button } from "@/components/shadcn/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/shadcn/card";
import { Input } from "@/components/shadcn/input";
import { Label } from "@/components/shadcn/label";
import { reactive } from "vue";
import { useDark } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
useDark();

const form = reactive({
  username: "",
  password: "",
});

const submit = () => {
  invoke("login", form)
    .then(() => {
      console.log("Login successful");
    })
    .catch((error) => {
      console.error("Logout failed:", error);
    });
};
const logout = () => {
  invoke("logout")
    .then(() => {
      console.log("Logout successful");
    })
    .catch((error) => {
      console.error("Logout failed:", error);
    });
};
</script>

<template>
  <div
    class="bg-muted flex min-h-svh flex-col items-center justify-center gap-6 p-6 md:p-10 transition-all"
  >
    <div class="flex w-full max-w-sm flex-col gap-6">
      <a href="#" class="flex items-center gap-2 self-center font-medium">
        <div
          class="bg-primary text-primary-foreground flex size-6 items-center justify-center rounded-md"
        >
          <GalleryVerticalEnd class="size-4" />
        </div>
        Chrm Rev
      </a>
      <div class="flex flex-col gap-6">
        <Card>
          <CardHeader class="text-center">
            <CardTitle class="text-xl">登录</CardTitle>
            <CardDescription> </CardDescription>
          </CardHeader>
          <CardContent>
            <div class="grid gap-6">
              <div class="grid gap-6">
                <div class="grid gap-3">
                  <Label for="email">用户名</Label>
                  <Input
                    placeholder="请输入"
                    v-model="form.username"
                    required
                  />
                </div>
                <div class="grid gap-3">
                  <div class="flex items-center">
                    <Label for="password">密码</Label>
                  </div>
                  <Input
                    type="password"
                    v-model="form.password"
                    required
                    placeholder="请输入"
                  />
                </div>
                <Button class="w-full" @click="submit">立即登录</Button>
                <Button class="w-full" @click="logout">退出登录</Button>
              </div>
            </div>
          </CardContent>
        </Card>
        <Footer></Footer>
      </div>
    </div>
  </div>
</template>
