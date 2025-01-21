<script lang="ts">
	import { onMount } from 'svelte'
	import Confetti from 'svelte-confetti'

	interface Props {
		rating?: number
		maxRating?: number
		displayRating?: boolean
		isInteractive?: boolean
	}

	let { rating = $bindable(1), maxRating = 5, displayRating = true, isInteractive = false }: Props = $props()

	let stars = $state<boolean[]>([])
	let displayConfetti = $state(false)

	$effect(() => {
		stars = Array.from({ length: maxRating }, (_, i) => i < rating)
	})

	const recompute = (index: number) => {
		rating = index + 1
		if (rating === maxRating) displayConfetti = true
		else displayConfetti = false
		for (let i = 0; i <= maxRating - 1; i++) stars[i] = i < rating
	}
</script>

<div class="rating-wrapper">
	{#each stars as isFilled, index}
		<button
			type="button"
			class="star {isFilled ? '' : 'empty'} {isInteractive ? 'interactive' : ''}"
			aria-label="Rate {index + 1} stars"
			disabled={!isInteractive}
			onclick={() => recompute(index)}
			onkeydown={e => (e.key === 'Enter' || e.key === ' ') && recompute(index)}
		>
			&#9733;
		</button>
	{/each}
	{#if displayRating}
		<span>{rating}/{maxRating}</span>
	{/if}
	{#if displayConfetti && isInteractive}
		<Confetti cone x={[0.25, 2]} />
	{/if}
</div>

<style>
	.rating-wrapper {
		width: fit-content;
		display: flex;
		align-items: baseline;
	}
	.star {
		font-size: 1.4rem;
		color: var(--primary-main);
		padding: 0.2rem;
		background-color: transparent;
		border: none;
		cursor: auto;
	}

	.star.interactive {
		cursor: pointer;
	}
	.star.interactive:hover {
		color: var(--primary-dark);
	}
	.star.empty.interactive:hover {
		color: var(--primary-light);
	}
	.star.empty {
		color: lightgray;
	}
	span {
		margin-left: 0.7rem;
	}
</style>
