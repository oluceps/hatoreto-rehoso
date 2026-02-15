import type { Component } from "solid-js";
import { createEffect, createSignal, onCleanup, onMount } from "solid-js";
import Chart from "chart.js/auto";
import { getHeartbeat } from "./apis/index";

const App: Component = () => {
	const [heartbeat, setHeartbeat] = createSignal<string>("~");
    const [minRate, setMinRate] = createSignal<number | null>(null);
    const [maxRate, setMaxRate] = createSignal<number | null>(null);
    
	let chartCanvas: HTMLCanvasElement | undefined;
	let chartInstance: Chart | null = null;
    const maxDataPoints = 60; // Show a bit more history (1 min if 1Hz)

	onMount(() => {
		getHeartbeat((data: string) => {
			setHeartbeat(data);
		});

		if (chartCanvas) {
            const ctx = chartCanvas.getContext('2d');
            if (ctx) {
                chartInstance = new Chart(ctx, {
                    type: "line",
                    data: {
                        labels: Array(maxDataPoints).fill(""),
                        datasets: [
                            {
                                data: Array(maxDataPoints).fill(null),
                                borderColor: "#e63f66", // cerise-red-500
                                backgroundColor: "rgba(230, 63, 102, 0.1)",
                                borderWidth: 2,
                                pointRadius: 0,
                                tension: 0.3,
                                fill: true,
                            },
                        ],
                    },
                    options: {
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {
                            legend: { display: false },
                            tooltip: { enabled: false }
                        },
                        scales: {
                            x: { display: false },
                            y: { 
                                display: true,
                                position: 'right',
                                grid: {
                                    color: "rgba(128, 27, 64, 0.05)", // cerise-red-900 with low opacity
                                },
                                border: { display: false },
                                ticks: {
                                    color: "#801b40", // cerise-red-900
                                    font: {
                                        family: 'monospace',
                                        size: 10,
                                        weight: 'bold'
                                    },
                                    stepSize: 20,
                                    callback: (value) => value
                                },
                                suggestedMin: 50,
                                suggestedMax: 150
                            },
                        },
                        animation: false,
                        events: [],
                    },
                });
            }
		}
	});

    onCleanup(() => {
        if (chartInstance) {
            chartInstance.destroy();
        }
    });

	createEffect(() => {
        const rateStr = heartbeat();
        if (rateStr === "~") return;

        const val = parseInt(rateStr, 10);
        if (isNaN(val)) return;

        if (minRate() === null || val < minRate()!) setMinRate(val);
        if (maxRate() === null || val > maxRate()!) setMaxRate(val);

		if (chartInstance) {
            const dataset = chartInstance.data.datasets[0];
            const data = dataset.data;
            
            data.push(val);
            if (data.length > maxDataPoints) {
                data.shift();
            }
            chartInstance.update();
        }
	});

	return (
		<div class="w-screen h-screen overflow-hidden bg-white flex flex-col items-center justify-center p-6">
            
            <div class="w-full max-w-md space-y-8">
                
                {/* Header Section */}
                <div class="flex justify-between items-center px-2">
                    <div class="flex flex-col">
                        <span class="text-cerise-red-900/40 text-[10px] font-bold tracking-widest leading-none">Real-time Pulse Monitor</span>
                    </div>
                    <div class="flex items-center space-x-2 bg-cerise-red-50 px-3 py-1 rounded-full border border-cerise-red-100">
                        <div class={`w-2 h-2 rounded-full ${heartbeat() === "~" ? 'bg-cerise-red-200' : 'bg-cerise-red-500 animate-pulse'}`}></div>
                        <span class="text-cerise-red-900/60 text-[9px] font-black uppercase tracking-wider">
                            {heartbeat() === "~" ? 'Offline' : 'Live'}
                        </span>
                    </div>
                </div>

                {/* Main Value Display */}
                <div class="relative group">
                    <div class="absolute inset-0 bg-cerise-red-500 blur-3xl opacity-10 group-hover:opacity-20 transition-opacity"></div>
                    <div class="relative bg-white border-2 border-cerise-red-50 shadow-[0_20px_50px_rgba(230,63,102,0.1)] rounded-[2.5rem] p-10 flex flex-col items-center">
                        <div class="absolute top-6 left-8">
                            <svg xmlns="http://www.w3.org/2000/svg" class="text-cerise-red-500 animate-[bounce_2s_infinite]" width="32" height="32" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                                <path d="M19.5 13.572l-7.5 7.428l-2.896 -2.868m-6.117 -8.104a5 5 0 0 1 9.013 -3.022a5 5 0 1 1 7.5 6.572" />
                                <path d="M3 13h2l2 3l2 -6l1 3h3" />
                            </svg>
                        </div>
                        
                        <div class="flex flex-col items-center">
                            <span class="text-9xl font-mono font-black text-cerise-red-900 tabular-nums tracking-tighter">
                                {heartbeat()}
                            </span>
                            <span class="text-cerise-red-500/50 font-black text-sm uppercase tracking-[0.4em] -mt-4">Beats / Min</span>
                        </div>
                    </div>
                </div>

                {/* Statistics & Chart Section */}
                <div class="bg-cerise-red-50/50 rounded-3xl p-6 border border-cerise-red-100/50">
                    
                    {/* Mini Stats Grid */}
                    <div class="grid grid-cols-2 gap-4 mb-6">
                        <div class="bg-white rounded-2xl p-4 shadow-sm border border-cerise-red-50 flex flex-col">
                            <span class="text-cerise-red-900/40 text-[9px] font-black uppercase tracking-widest">Session Min</span>
                            <span class="text-cerise-red-700 font-mono text-2xl font-black">{minRate() ?? "--"}</span>
                        </div>
                        <div class="bg-white rounded-2xl p-4 shadow-sm border border-cerise-red-50 flex flex-col items-end">
                            <span class="text-cerise-red-900/40 text-[9px] font-black uppercase tracking-widest">Session Max</span>
                            <span class="text-cerise-red-700 font-mono text-2xl font-black">{maxRate() ?? "--"}</span>
                        </div>
                    </div>

                    {/* Chart with Axis */}
                    <div class="h-36 w-full relative pr-2">
                        <canvas ref={chartCanvas} />
                    </div>

                    <div class="mt-4 flex justify-center">
                        <div class="h-1 w-12 bg-cerise-red-200 rounded-full"></div>
                    </div>
                </div>

            </div>
		</div>
	);
};

export default App;
