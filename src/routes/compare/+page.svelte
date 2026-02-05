<script lang="ts">
	import GitCompareArrowsIcon from '@lucide/svelte/icons/git-compare-arrows';
</script>

<div class="space-y-6">
	<div class="flex items-center gap-3">
		<GitCompareArrowsIcon class="text-primary size-6" />
		<div>
			<h2 class="text-lg font-semibold">Compare</h2>
			<p class="text-muted-foreground text-sm">Compare your gameplay against professional players</p>
		</div>
	</div>

	<div class="bg-card rounded-lg border p-6">
		<p class="text-muted-foreground text-sm">
			Select a demo and a pro player to see a side-by-side comparison of positioning, utility usage, and
			decision making.
		</p>
	</div>
</div>
