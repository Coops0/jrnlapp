import { invoke } from '@tauri-apps/api/core'

export async function requestSignin(): Promise<unknown> {
  return await invoke('plugin:google-signin|request_signin');
}

export async function logout(): Promise<void> {
  return await invoke('plugin:google-signin|logout');
}