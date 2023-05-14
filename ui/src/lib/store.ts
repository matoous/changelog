import { writable, readable } from 'svelte/store';

export const APIserver = readable('http://127.0.0.1:8080/api');
