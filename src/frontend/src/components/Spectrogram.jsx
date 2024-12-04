import React, { useEffect, useRef, useState } from "react";

const Spectrogram = () => {
  const containerRef = useRef(null);
  const waterfallRef = useRef(null);
  const [bluetoothActive, setBluetoothActive] = useState(false);
  const [wifiActive, setWifiActive] = useState(false);
  const [liveData, setLiveData] = useState(false);
  const [wifiBand, setWifiBand] = useState(null);

  useEffect(() => {
    const createColorMap = () => {
      const colMap = [];
      for (let i = 0; i < 256; i++) {
        const r = Math.min(255, Math.max(0, i > 128 ? (i - 128) * 2 : 0));
        const g = Math.min(255, Math.max(0, i > 64 ? i * 2 : 0));
        const b = Math.min(255, Math.max(0, 255 - i * 2));
        colMap.push([r, g, b, 255]);
      }
      return colMap;
    };

    const initializeWaterfall = () => {
      const colMap = createColorMap();
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");

      const width = 500;
      const height = 200;
      canvas.width = width;
      canvas.height = height;

      if (containerRef.current) {
        containerRef.current.appendChild(canvas);
      }

      const drawLine = (buffer) => {
        if (!buffer || buffer.length === 0) return;
        const imgData = ctx.createImageData(width, 1);
        for (let i = 0; i < width; i++) {
          const intensity = buffer[i] || 0;
          const [r, g, b, a] = colMap[intensity];
          const index = i * 4;
          imgData.data[index] = r;
          imgData.data[index + 1] = g;
          imgData.data[index + 2] = b;
          imgData.data[index + 3] = a;
        }
        ctx.putImageData(imgData, 0, height - 1);
        ctx.drawImage(canvas, 0, 1, width, height - 1, 0, 0, width, height - 1);
      };

      waterfallRef.current = { canvas, ctx, drawLine, clear: () => ctx.clearRect(0, 0, width, height) };

      return () => {
        if (containerRef.current && waterfallRef.current?.canvas) {
          containerRef.current.removeChild(waterfallRef.current.canvas);
        }
      };
    };

    const cleanup = initializeWaterfall();

    return () => cleanup();
  }, []);

  useEffect(() => {
    const simulateFFT = () => {
      const buffer = new Uint8Array(500).fill(0);
      const frequencies = {
        bluetooth: [2400, 2483],
        wifi: [
          [2400, 2500],
          [3600, 3700],
          [5000, 5800],
        ],
      };

      const addSignal = (range, buffer) => {
        const randomFreq = Math.random() * (range[1] - range[0]) + range[0];
        const index = Math.floor(((randomFreq - 2400) / 5000) * buffer.length);

        for (let i = -2; i <= 2; i++) {
          const targetIndex = index + i;
          if (targetIndex >= 0 && targetIndex < buffer.length) {
            buffer[targetIndex] = Math.max(buffer[targetIndex], 255);
          }
        }
      };

      if (!liveData) {
        if (bluetoothActive) {
          addSignal(frequencies.bluetooth, buffer);
        }

        if (wifiActive && wifiBand) {
          addSignal(wifiBand, buffer);
        }

        for (let i = 0; i < buffer.length; i++) {
          buffer[i] = Math.max(buffer[i], Math.random() * 50);
        }

        if (waterfallRef.current) {
          waterfallRef.current.drawLine(buffer);
        }
      }
    };

    const interval = setInterval(simulateFFT, 1000 / 30);

    return () => clearInterval(interval);
  }, [bluetoothActive, wifiActive, wifiBand, liveData]);

  const handleClear = () => {
    waterfallRef.current?.clear();
  };

  const handleWifiToggle = () => {
    setWifiActive(!wifiActive);

    if (!wifiActive) {
      const wifiBands = [
        [2400, 2500],
        [3600, 3700],
        [5000, 5800],
      ];
      setWifiBand(wifiBands[Math.floor(Math.random() * wifiBands.length)]);
    } else {
      setWifiBand(null);
    }
  };

  return (
    <div>
      <div ref={containerRef} style={{ width: "500px", height: "200px", border: "1px solid black" }}></div>
      <div style={{ marginTop: "10px" }}>
        <button onClick={() => setBluetoothActive(!bluetoothActive)} style={{ marginRight: "10px" }}>
          {bluetoothActive ? "Disable Bluetooth" : "Enable Bluetooth"}
        </button>
        <button onClick={handleWifiToggle} style={{ marginRight: "10px" }}>
          {wifiActive ? "Disable Wi-Fi" : "Enable Wi-Fi"}
        </button>
        <button onClick={() => setLiveData(!liveData)} style={{ marginRight: "10px" }}>
          {liveData ? "Switch to Demo Data" : "Switch to Live Data"}
        </button>
        <button onClick={handleClear}>Clear</button>
      </div>
    </div>
  );
};

export default Spectrogram;
