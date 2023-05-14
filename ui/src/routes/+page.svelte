<script lang="ts">
	import { Editor } from 'bytemd';
	import type { Entry as EntryT } from '$lib/types/entry';
	import Entry from './Entry.svelte';
	import 'bytemd/dist/index.css';

	let value = '';

	export let data: {
		entries: EntryT[];
	};
</script>

<div class="container">
	<Editor
		{value}
		on:change={(e) => {
			value = e.detail.value;
		}}
	/>
</div>

<div class="timeline">
	{#each data.entries as entry}
		<Entry {entry} />
	{/each}
</div>

<style>
	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.timeline {
		margin: 0 auto;
		max-width: 1000px;
	}
</style>
