import type { Component } from "solid-js";
import { Show, createEffect, createSignal, lazy } from "solid-js";
import { getHeartbeat } from "./apis/index";

const App: Component = () => {
	const [heartbeat, setHeartbeat] = createSignal<number>(50);
	createEffect(async () => {
		const interval = setInterval(async () => {
			setHeartbeat(await getHeartbeat());
		}, 1000);
		return () => clearInterval(interval);
	});

	return (
		<div class="bg-cerise-red-150 h-screen w-screen grid grid-rows-5 grid-cols-3">
			<div class="flex overflow-auto justify-end items-center shadow-lg row-start-2 row-span-2 col-start-2 col-span-1 bg-cerise-red-50 rounded-xl">
				<div class="w-full flex justify-start items-center ml-11">
					<div class="bg-cerise-red-50">
						{/* biome-ignore lint/a11y/noSvgWithoutTitle: <explanation> */}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="icon icon-tabler icon-tabler-activity-heartbeat"
							width="100"
							height="100"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="#b21e4b"
							fill="none"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path stroke="none" d="M0 0h24v24H0z" fill="none" />
							<path d="M3 12h4.5l1.5 -6l4 12l2 -9l1.5 3h4.5" />
						</svg>
					</div>
				</div>

				<span class="mr-10 shadow-sm font-mono text-6xl text-cerise-red-50 bg-cerise-red-400 pt-6 px-6 py-3 rounded-md">
					<div class="animate-pulse">{heartbeat()}</div>
				</span>
			</div>
		</div>
	);
};

export default App;
