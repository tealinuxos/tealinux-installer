<script>
	import { modalStore, closeModal } from '$lib/stores/modalStore';

	let modal;

	$: {
        modal = $modalStore;
        onMount();
    }

    let interval = null;
    let disabled = true;
    let countdown = null;

	const handleConfirm = () => {
        clearInterval(interval);
        interval = null;
        if (modal.onConfirm) modal.onConfirm();
        closeModal();
	};

	const handleCancel = () => {
        clearInterval(interval);
        interval = null;
		if (modal.onCancel) modal.onCancel();
		closeModal();
	};

    const onMount = () => {
        countdown = modal?.countdown ?? 0;
        disabled = true;
        if (!interval && countdown !== 0) {
            disabled = true;
            interval = setInterval(() => {
                if (countdown > 0) {
                    countdown -= 1;
                } 
                else if (countdown === 0) {
                    clearInterval(interval);
                    disabled = false;
                    interval = null;
                    countdown = -1;
                }
            }, 1000)
        }

        if (!countdown || countdown === -1) disabled = false;
    };
</script>

{#if modal.isOpen}
	{#if modal.type === 'error'}
      <div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
            <div
                style="-webkit-backdrop-filter: blur(10px); backdrop-filter: blur(10px)"
                class="absolute inset-0 bg-black/50"
                on:click={handleCancel}
            ></div>
            
            <div
                class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-[10px] border border-[#f87171] shadow-[0_0_30px_rgba(239,68,68,0.25)] overflow-auto z-90"
            >
                <div class="w-full p-6 z-10">
                    <!-- Warning Icon -->
                    <div class="flex justify-center mb-4">
                        <div class="relative">
                            <div class="absolute inset-0 rounded-full bg-red-500 blur-md opacity-30 animate-pulse"></div>
                            <svg
                                width="64"
                                height="64"
                                viewBox="0 0 24 24"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                                class="text-red-400 relative z-10"
                            >
                                <path
                                    d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM13 17H11V15H13V17ZM13 13H11V7H13V13Z"
                                    fill="currentColor"
                                />
                            </svg>
                        </div>
                    </div>

                    <!-- Title -->
                    <h2 class="text-2xl font-bold text-center mb-4 bg-gradient-to-r from-red-400 to-red-600 bg-clip-text text-transparent">
                        {$modalStore.title || 'Warning'}
                    </h2>

                    <!-- Content -->
                    <div class="text-gray-300/90 text-center text-sm leading-relaxed max-w-xs mx-auto mb-6">
                        {@html $modalStore.content || 'This action cannot be undone.'}
                    </div>

                    <!-- Buttons -->
                    <div class="flex gap-2 mt-4">
                        <button
                            class="w-full px-4 py-2 rounded text-white border border-red-500/50 hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(239,68,68,0.8)] disabled:opacity-50 transition-colors"
                            on:click={handleCancel}
                        >
                            Cancel
                        </button>
                        <button
                            class="w-full px-4 py-2 rounded text-white bg-red-600/80 border border-red-500/50 hover:bg-red-700 active:shadow-[0_0_7.167px_rgba(239,68,68,0.8)] disabled:opacity-50 transition-colors"
                            on:click={handleConfirm}
                            disabled={disabled}
                        >
                            Confirm {#if countdown >= 0}({countdown}){/if}
                        </button>
                    </div>
                </div>
            </div>
    </div>
	{:else}
		<!-- Original Modal (all other types) -->
		<div class="fixed inset-0 z-50 overflow-y-auto">
			<div
				class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0"
			>
				<!-- Background overlay -->
				<div class="fixed inset-0 transition-opacity" on:click={closeModal}>
					<div class="absolute inset-0 bg-gray-500 opacity-75"></div>
				</div>

				<div
					class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full"
					role="dialog"
					aria-modal="true"
					aria-labelledby="modal-headline"
				>
					<div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
						<div class="sm:flex sm:items-start">
							<!-- Icon based on modal type -->
							{#if modal.type === 'success'}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-green-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-green-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M5 13l4 4L19 7"
										/>
									</svg>
								</div>
							{:else if modal.type === 'warning'}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-yellow-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-yellow-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
										/>
									</svg>
								</div>
							{:else}
								<div
									class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-blue-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-blue-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
										/>
									</svg>
								</div>
							{/if}

							<div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
								<h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-headline">
									{modal.title}
								</h3>
								<div class="mt-2">
									<p class="text-sm text-gray-500">
										{@html modal.content}
									</p>
								</div>
							</div>
						</div>
					</div>
					<div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
						<button
							type="button"
							class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-600 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm transition-colors duration-200"
							on:click={handleConfirm}
						>
							OK
						</button>
						{#if modal.showCancel}
							<button
								type="button"
								class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm transition-colors duration-200"
								on:click={closeModal}
							>
								Cancel
							</button>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
{/if}