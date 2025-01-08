<script lang="ts">
	interface Props {
		entities: { id: number; name: string }[]
		getHref(id: number): string
	}

	let { entities, getHref }: Props = $props()
</script>

<ul>
	{#each entities as entity}
		<li>
			<a href={getHref(entity.id)}>{entity.name}</a>
		</li>
	{/each}
</ul>

<style>
	ul {
		list-style: none;
		padding: 0;
	}

	li {
		margin-bottom: 0.5rem;
	}

	a {
		text-decoration: none;
		color: var(--secondary-main);
	}

	a:hover {
		text-decoration: underline;
	}
</style>
