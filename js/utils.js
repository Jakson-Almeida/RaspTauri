import { invoke } from '@tauri-apps/api/tauri';

window.__TAURI__ = window.__TAURI__ || {};
window.__TAURI__.tauri = window.__TAURI__.tauri || {};

window.__TAURI__.tauri.getMouseCoordinates = function() {
    return new Promise((resolve) => {
        document.addEventListener('mousemove', (event) => {
            resolve({ x: event.clientX, y: event.clientY });
        }, { once: true });
    });
};


export function getFixedCoordinates() {
  return Promise.resolve({ x: 100, y: 200 });
}

export function getMouseCoordinates() {
    return new Promise((resolve) => {
        document.addEventListener('mousemove', (event) => {
            resolve({ x: event.clientX, y: event.clientY });
        }, { once: true });
    });
}


