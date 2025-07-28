import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [volume, setVolume] = useState(0);

  useEffect(() => {
    async function fetchVolume() {
      const initialVolume = await invoke("get_volume");
      setVolume(initialVolume as number);
    }
    fetchVolume();
  }, []);

  async function handleVolumeChange(newVolume: number) {
    setVolume(newVolume);
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
