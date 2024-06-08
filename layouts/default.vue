<template>
  <div class="flex flex-col lg:flex-row h-screen bg-base-200">
    <!-- Sidebar -->
    <div class="w-full lg:w-64 bg-base-300">
      <div class="flex items-center justify-center h-16">
        <a href="/" class="btn btn-ghost text-xl w-full lg:w-60">
          <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="85 100 330 300" class="fill-current">
            <path
                d="M189.8 127.7c-7.6 2.4-13.8 4.8-13.8 5.2 0 .4 3.7 1.9 8.3 3.4l8.3 2.7h121.9l8-2.7c4.4-1.4 8.4-2.9 8.8-3.3.4-.4-6.1-2.8-14.5-5.3l-15.3-4.7-49 .1-49 .1-13.7 4.5zM150.6 150.9c-3 4.7-5.3 8.7-5.1 8.9.1.2 2.1-1 4.2-2.5 12.3-8.7 15.2-10.8 15.3-11.3 0-.3-2-1.2-4.5-2.1l-4.4-1.6-5.5 8.6zM346.2 143.7c-3.5 1.2-4.1 1.8-3.1 2.9 1.3 1.6 19 13.6 19.4 13.2.4-.4-10.9-17.8-11.5-17.7-.3 0-2.4.7-4.8 1.6zM168.5 165c-6.1 4.4-15 10.8-20 14.2-4.9 3.4-10.8 7.6-13 9.3-2.2 1.8-8 5.9-12.8 9.1-7.2 4.7-8.5 6-7 6.6 2.9 1.1 45.2 12.9 51.8 14.4 3.3.7 9.5 2.4 13.8 3.8l7.7 2.4v-2.6c0-2.5-6.4-55-7.6-62-.2-1.7-.8-3.2-1.2-3.2-.4 0-5.7 3.6-11.7 8zM199 159.2c0 2.8 7.9 63.9 8.5 65.5.6 1.9 93.5 1.9 93.5 0 .1-5.3 5.7-53.5 7.6-64.5l.5-3.2H199v2.2zM324.6 173.7c-1 9.3-2.7 23.8-3.7 32.3-1 8.5-1.9 16.2-1.9 17.1 0 1.3 3.7.6 21.3-4.2 23.1-6.2 52.8-14.6 53.3-15.2.2-.2-6.3-4.9-14.4-10.5-8.1-5.6-19.4-13.5-25.2-17.7-16.8-11.9-24.9-17.5-26.3-18.1-1-.3-1.7 3.6-3.1 16.3zM120 224.6c.8 1 6.4 7.5 12.4 14.4s14.1 16.2 18 20.8c3.9 4.6 10.3 12 14.2 16.5 3.8 4.5 10.1 12 13.9 16.7 3.8 4.7 7.8 9.4 8.8 10.5 1 1.1 4.4 5.1 7.5 9 3.1 3.8 9.6 11.5 14.3 17 4.7 5.5 13.8 16.5 20.3 24.5s12.3 14.9 12.8 15.5c1.2 1.4-1.6-6.4-13.8-37.5-24.9-63.6-31.2-79.8-32.8-84.3-1-2.6-1.9-4.7-2.1-4.7-2.1 0-52.9-13.7-73-19.7-1.8-.5-1.9-.4-.5 1.3zM359 231c-15.1 4.2-31 8.5-35.3 9.5-5.6 1.3-8.2 2.4-9.1 3.9-.7 1.2-3.6 7.9-6.4 15.1-2.9 7.1-10.4 26.3-16.8 42.5s-14.7 37.4-18.6 47c-3.9 9.6-7.3 18.2-7.6 19-1 2.7 3.7-2.1 10.1-10.4 3.4-4.3 8.9-11.1 12.3-15 3.3-3.9 8.3-9.8 11-13.2 6.8-8.4 34.6-41.6 41-48.9 14.9-17 21.3-24.5 25.4-29.5 2.4-3 6.9-8.2 10-11.5 9.4-10.3 14.2-16.5 12.8-16.4-.7 0-13.7 3.6-28.8 7.9zM215.1 244c-1.9.5-1.9.6.5 6.5 15.9 40.6 22 56.3 24.4 63 1.6 4.4 4.7 12.7 7 18.5 2.3 5.8 4.6 11.7 5.2 13.2.5 1.6 1.4 2.6 1.8 2.3.5-.3 2.1-3.8 3.6-7.8 6.5-17.3 33.6-86.2 37.1-94.4.4-1-8.2-1.3-38.5-1.5-21.5-.2-40-.1-41.1.2z"/>
          </svg>
          GEMS
        </a>
      </div>
      <nav class="flex flex-col p-4">
        <a href="#" class="flex items-center py-2 text-base-content hover:bg-base-100 rounded-lg">
          <span class="mr-4">🏠</span>Home
        </a>
        <a href="#" class="flex items-center py-2 text-base-content hover:bg-base-100 rounded-lg">
          <span class="mr-4">📦</span> Environments
        </a>
        <a href="#" class="flex items-center py-2 text-base-content hover:bg-base-100 rounded-lg">
          <span class="mr-4">🔧</span> Settings
        </a>
      </nav>
    </div>
    <div class="flex flex-col flex-1">
      <!-- Top Bar -->
      <div class="flex items-center justify-between p-4 bg-base-100 shadow-md">
        <h1 class="text-2xl font-bold">Dashboard</h1>
        <div class="flex items-center space-x-4">
          <!-- Theme Dropdown -->
          <div class="dropdown dropdown-end tooltip tooltip-left" data-tip="Change Theme">
            <div tabindex="0" role="button" class="btn btn-ghost flex items-center space-x-1">
              <p style="font-size: 11pt;">Theme</p>
              <svg width="10px" height="10px" class="fill-current" xmlns="http://www.w3.org/2000/svg"
                   viewBox="0 0 2048 2048">
                <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
              </svg>
            </div>
            <div
                class="dropdown-content bg-base-200 rounded-box top-px h-[28.6rem] max-h-[calc(100vh-10rem)] w-56 overflow-y-auto border border-white/5 shadow-2xl outline outline-1 outline-black/5 mt-16">
              <div class="grid grid-cols-1 gap-3 p-3">
                <button @click="colorMode.preference = theme" v-for="theme in themes" :key="theme"
                        class="outline-base-content text-start outline-offset-4" :data-set-theme="theme">
                  <span :data-theme="theme"
                        class="bg-base-100 rounded-btn text-base-content block w-full cursor-pointer font-sans">
                    <span class="grid grid-cols-5 grid-rows-3">
                      <span class="col-span-5 row-span-3 row-start-1 flex items-center gap-2 px-4 py-3">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                             fill="currentColor" class="invisible h-3 w-3 shrink-0">
                          <path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"></path>
                        </svg>
                        <span class="flex-grow text-sm">{{ theme }}</span>
                        <span class="flex h-full shrink-0 flex-wrap gap-1">
                          <span class="bg-primary rounded-badge w-2"></span>
                          <span class="bg-secondary rounded-badge w-2"></span>
                          <span class="bg-accent rounded-badge w-2"></span>
                          <span class="bg-neutral rounded-badge w-2"></span>
                        </span>
                      </span>
                    </span>
                  </span>
                </button>
              </div>
            </div>
          </div>
          <div class="tooltip tooltip-bottom" data-tip="Login">
            <a href="/login" class="btn btn-ghost">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current">
                <path
                    d="M14 19v-.083c-1.178.685-2.542 1.083-4 1.083-4.411 0-8-3.589-8-8s3.589-8 8-8c1.458 0 2.822.398 4 1.083v-2.245c-1.226-.536-2.576-.838-4-.838-5.522 0-10 4.477-10 10s4.478 10 10 10c1.424 0 2.774-.302 4-.838v-2.162zm4-9.592l2.963 2.592-2.963 2.592v-1.592h-8v-2h8v-1.592zm-2-4.408v4h-8v6h8v4l8-7-8-7z"/>
              </svg>
            </a>
          </div>
        </div>
      </div>
      <!-- Main Section -->
      <div class="p-4 flex-1 overflow-y-auto">
        <slot/>
      </div>
    </div>
  </div>
</template>

<script setup>
import {_themes as daisyThemes} from '#tailwind-config/daisyui'

const colorMode = useColorMode();
const themes = new Set(daisyThemes);
</script>
<style>
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  border-radius: 100px;
}

::-webkit-scrollbar-thumb {
  border-radius: 100px;
  background-color: currentColor;
}
</style>
