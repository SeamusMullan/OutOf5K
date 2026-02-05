<script lang="ts">
	import LayoutDashboardIcon from '@lucide/svelte/icons/layout-dashboard';
</script>

<div class="space-y-6">
	<div class="flex items-center gap-3">
		<LayoutDashboardIcon class="text-primary size-6" />
		<div>
			<h2 class="text-lg font-semibold">Dashboard</h2>
			<p class="text-muted-foreground text-sm">Overview of your CS2 gameplay analysis</p>
		</div>
	</div>

	<div class="grid gap-4 sm:grid-cols-3">
		<div class="bg-card rounded-lg border p-4">
			<p class="text-muted-foreground text-sm font-medium">Total Demos</p>
			<p class="text-3xl font-bold">0</p>
		</div>
		<div class="bg-card rounded-lg border p-4">
			<p class="text-muted-foreground text-sm font-medium">K/D Ratio</p>
			<p class="text-3xl font-bold">&mdash;</p>
		</div>
		<div class="bg-card rounded-lg border p-4">
			<p class="text-muted-foreground text-sm font-medium">Win Rate</p>
			<p class="text-3xl font-bold">&mdash;</p>
		</div>
	</div>

	<div class="bg-card rounded-lg border p-6">
		<h3 class="mb-4 font-medium">Recent Activity</h3>
		<p class="text-muted-foreground text-sm">No demos analyzed yet. Import a demo to get started.</p>
	</div>
</div>
