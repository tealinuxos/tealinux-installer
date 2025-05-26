import { writable } from 'svelte/store';

export const modalStore = writable({
	isOpen: false,
	title: '',
	content: '',
	type: 'info', // 'info', 'success', 'error', 'warning'
	onConfirm: null,
	showCancel: false
});

export function showModal(options) {
	modalStore.set({
		isOpen: true,
		title: options.title || '',
		content: options.content || '',
		type: options.type || 'info',
		onConfirm: options.onConfirm || null,
		showCancel: options.showCancel || false
	});
}

export function closeModal() {
	modalStore.update((store) => {
		return { ...store, isOpen: false };
	});
}
