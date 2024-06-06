<template>
  <div class="drawer">
    <input id="sidebar" type="checkbox" class="drawer-toggle"/>
    <div class="drawer-content flex flex-col">
      <div class="w-full navbar bg-base-100">
        <div class="flex-1">
          <!-- Sidebar icon and label -->
          <div class="tooltip tooltip-right" data-tip="Open Sidebar">
            <label for="sidebar" aria-label="open sidebar" class="btn btn-square btn-ghost">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                   class="inline-block w-6 h-6 stroke-current">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 6h16M4 12h16M4 18h16"></path>
              </svg>
            </label>
          </div>
        </div>
        <div class="flex-none">
          <!-- Theme dropdown -->
          <div class="dropdown dropdown-end tooltip tooltip-left" data-tip="Change Theme">
            <div tabindex="0" role="button" class="btn btn-ghost">
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
          <!-- Login button -->
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
      <!-- Main content -->
      <div class="hero min-h-screen bg-base-200">
        <div class="hero-content text-center">
          <slot/>
        </div>
      </div>
    </div>
    <!-- Sidebar -->
    <div class="drawer-side" style="margin-top: 5vh">
      <label for="sidebar" aria-label="close sidebar" class="drawer-overlay"></label>
      <ul class="menu bg-base-200 w-56 min-h-screen rounded-box">
        <li>
          <h2 class="menu-title">Sidebar</h2>
          <ul>
            <li>
              <a href="/">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                     stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                Home
              </a>
            </li>
            <li>
              <a>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                     stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                </svg>
                Dashboard
              </a>
            </li>
            <li>
              <a>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                     stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                </svg>
                Overview
              </a>
            </li>
          </ul>
        </li>
      </ul>
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

.min-h-screen {
  min-height: 95vh;
  padding-bottom: 25rem;
}
</style>