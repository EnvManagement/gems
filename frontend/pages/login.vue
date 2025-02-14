<template>
  <div class="card min-w-96 p-8 space-y-6 shadow-2xl bg-base-100">
    <h2 class="text-3xl font-bold text-center">Login</h2>
    <form class="space-y-4">
      <div class="form-control">
        <label class="label">
          <span class="label-text">Email</span>
        </label>
        <input type="email" placeholder="you@example.com" class="input input-bordered w-full">
      </div>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Password</span>
        </label>
        <input type="password" placeholder="••••••••" class="input input-bordered w-full">
      </div>
      <div class="form-control">
        <label class="label cursor-pointer">
          <input type="checkbox" class="checkbox">
          <span class="label-text">Remember me</span>
        </label>
      </div>
      <div>
        <button class="btn btn-primary w-full">Login</button>
      </div>
      <div class="text-center">
        <a href="#" class="link link-hover">Forgot password?</a>
      </div>
    </form>
  </div>
</template>