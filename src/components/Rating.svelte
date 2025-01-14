<script lang="ts">
	interface Props {
		rating?: number
		maxRating?: number
		displayRating?: boolean
		isInteractive?: boolean
	}

	let { rating = 0, maxRating = 5, displayRating = true, isInteractive = false }: Props = $props()

	const getStars = () => {
		let stars = []
		for (let i = 1; i <= maxRating; i++) stars.push(i <= rating)
		return stars
	}
</script>

<div class="rating-wrapper">
	{#each getStars() as isFilled, index}
		<button
			type="button"
			class="star {isFilled ? '' : 'empty'} {isInteractive ? 'interactive' : ''}"
			aria-label="Rate {index + 1} stars"
			disabled={!isInteractive}
			onclick={() => (rating = index + 1)}
			onkeydown={e => (e.key === 'Enter' || e.key === ' ') && (rating = index + 1)}
		>
			&#9733;
		</button>
	{/each}
	{#if displayRating}
		<span>{rating}/{maxRating}</span>
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
