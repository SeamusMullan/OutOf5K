<script lang="ts">
	import { page } from '$app/state';
	import FileVideoIcon from '@lucide/svelte/icons/file-video';
</script>

<div class="space-y-6">
	<div class="flex items-center gap-3">
		<FileVideoIcon class="text-primary size-6" />
		<div>
			<h2 class="text-lg font-semibold">Demo Viewer</h2>
			<p class="text-muted-foreground text-sm">Viewing demo: {page.params.id}</p>
		</div>
	</div>

	<div class="bg-card rounded-lg border p-6">
		<p class="text-muted-foreground text-sm">
			Demo analysis and round-by-round breakdown will appear here.
		</p>
	</div>
</div>
