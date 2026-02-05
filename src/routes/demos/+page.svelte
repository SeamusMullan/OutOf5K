<script lang="ts">
	import FileVideoIcon from '@lucide/svelte/icons/file-video';
</script>

<div class="space-y-6">
	<div class="flex items-center gap-3">
		<FileVideoIcon class="text-primary size-6" />
		<div>
			<h2 class="text-lg font-semibold">Demos</h2>
			<p class="text-muted-foreground text-sm">Manage and analyze your CS2 demo files</p>
		</div>
	</div>

	<div class="flex min-h-[400px] items-center justify-center rounded-lg border-2 border-dashed p-12">
		<div class="text-center">
			<FileVideoIcon class="text-muted-foreground mx-auto mb-4 size-12" />
			<h3 class="mb-1 font-medium">No demos yet</h3>
			<p class="text-muted-foreground text-sm">
				Drop .dem files here or click to import
			</p>
		</div>
	</div>
</div>
