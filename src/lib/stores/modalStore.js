import { writable } from 'svelte/store';

export const modalStore = writable({
	isOpen: false,
	title: '',
	content: '',
	type: 'info', // 'info', 'success', 'error', 'warning'
	onConfirm: null,
	onCancel: null,
	showCancel: false,
    countdown: 0
});

export function showModal(options) {
	modalStore.set({
		isOpen: true,
		title: options.title || '',
		content: options.content || '',
		type: options.type || 'info',
		onConfirm: options.onConfirm || null,
		onCancel: options.onCancel || null,
		showCancel: options.showCancel || false,
        countdown: options.countdown || 0
	});
}

export function closeModal() {
	modalStore.update((store) => {
		return { ...store, isOpen: false };
	});
}
