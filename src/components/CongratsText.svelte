<script lang="ts">
	import Confetti from 'svelte-confetti'
	let { children } = $props()
</script>

<div class="container">
	<Confetti amount={100} infinite x={[1, 3]} />
</div>
<strong>{@render children?.()}</strong>

<style>
	@keyframes rotateColors {
		0% {
			background-position: 0% 0%;
		}
		50% {
			background-position: 100% 100%;
		}
		100% {
			background-position: 0% 0%;
		}
	}

	strong {
		padding-inline: 0.5rem;
		background-image: linear-gradient(to right, var(--primary-main), rgb(219, 4, 219), var(--secondary-light));
		background-clip: text;
		color: transparent;
		background-size: 80%;
		animation: rotateColors 3s linear infinite;
	}
</style>
