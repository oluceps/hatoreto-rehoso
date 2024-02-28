import type { Component } from "solid-js";
import { Show, createEffect, createSignal, lazy } from "solid-js";
import { getHeartbeat } from "./apis/index";

const App: Component = () => {
	const [heartbeat, setHeartbeat] = createSignal<string>("50");
	createEffect(async () => {
		const interval = setInterval(async () => {
			setHeartbeat(await getHeartbeat());
		}, 1000);
		return () => clearInterval(interval);
	});

	return (
		<div class="w-screen h-screen overflow-hidden">
			<div class="flex-row justify-end items-center h-auto bg-cerise-red-50" />
			<div class="bg-cerise-red-50 backdrop-filter-none h-full w-full grid grid-rows-7 md:grid-rows-8 grid-cols-3 md:grid-cols-5">
				<div class="flex overflow-auto justify-end items-center shadow-lg mx-4 md:row-start-3 md:row-span-3 md:col-start-2 md:col-span-3 row-start-3 row-span-3 col-start-1 col-span-3 bg-cerise-red-100 rounded-xl">
					<div class="animate-pulse w-full flex justify-start items-center ml-11">
						<div>
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

					<span class="mr-3 shadow-md font-mono text-6xl text-cerise-red-50 bg-cerise-red-400 px-6 pb-3 pt-5 rounded-md">
						<div class="">{heartbeat()}</div>
					</span>
					<span class="font-mono text-md text-neutral-400 bg-cerise-red-100 mr-4 mt-16">
						bpm
					</span>
				</div>
			</div>
		</div>
	);
};

export default App;
