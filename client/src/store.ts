import { writable, derived } from 'svelte/store';

export const apiData = writable([]);

export const posts = derived(apiData, ($apiData) => {
  if ($apiData) {
    // @ts-ignore
    return $apiData.map(post => ({title: post.title, text: post.url, url: post.text}));
  }
  return [];
});
