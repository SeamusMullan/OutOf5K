<script lang="ts">
	import SettingsIcon from '@lucide/svelte/icons/settings';
</script>

<div class="space-y-6">
	<div class="flex items-center gap-3">
		<SettingsIcon class="text-primary size-6" />
		<div>
			<h2 class="text-lg font-semibold">Settings</h2>
			<p class="text-muted-foreground text-sm">Configure application preferences</p>
		</div>
	</div>

	<div class="bg-card rounded-lg border p-6">
		<p class="text-muted-foreground text-sm">
			Settings for CS2 demo path, Steam ID, parser configuration, and display preferences
			will be available here.
		</p>
	</div>
</div>
