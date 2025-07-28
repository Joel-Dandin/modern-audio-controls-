import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [volume, setVolume] = useState(0);

  useEffect(() => {
    const unlisten = listen("volume-changed", (event) => {
      setVolume(event.payload as number);
    });

    invoke("get_volume").then((initialVolume) => {
      setVolume(initialVolume as number);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  async function handleVolumeChange(newVolume: number) {
    await invoke("set_volume", { volume: newVolume });
  }

  return (
    <main className="container">
      <h1>System Volume Control</h1>

      <div className="row">
        <input
          type="range"
          min="0"
          max="100"
          value={volume}
          onChange={(e) => handleVolumeChange(parseInt(e.currentTarget.value))}
        />
        <span>{volume}%</span>
      </div>
    </main>
  );
}

export default App;
