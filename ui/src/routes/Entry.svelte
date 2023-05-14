<script lang="ts">
	import type { Entry } from '$lib/types/entry';
	import Time from 'svelte-time';

	export let entry: Entry;
</script>

<div class="entry">
	<div class="meta">
		<Time timestamp={entry.created_at} />
	</div>

	<div class="body">
		<div class="content">
			{@html entry.html}
		</div>
		{#if entry.tags}
			<div class="tags">
				{#each entry.tags as tag}
					<div class="tag">
						{tag}
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.entry {
		display: flex;
		flex-direction: row;
	}

	.meta {
		width: 8rem;
		margin-right: 16px;
	}

	.content > :first-child {
		margin-top: 0;
	}

	.tags {
		display: flex;
		flex-direction: row;
	}

	.tag {
		margin-right: 8px;
	}
</style>
