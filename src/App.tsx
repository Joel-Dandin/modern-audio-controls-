import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [volume, setVolume] = useState(0);
  const [mediaPosition, setMediaPosition] = useState(0);
  const [mediaDuration, setMediaDuration] = useState(0);
  const [mediaTitle, setMediaTitle] = useState("");
  const [mediaCoverArt, setMediaCoverArt] = useState<string | undefined>(undefined);

  useEffect(() => {
    const unlistenVolume = listen("volume-changed", (event) => {
      setVolume(event.payload as number);
    });

    invoke("get_volume").then((initialVolume) => {
      setVolume(initialVolume as number);
    });

    const interval = setInterval(async () => {
      const state = await invoke("get_media_state");
      if (state) {
        const [position, duration] = state as [number, number];
        setMediaPosition(position);
        setMediaDuration(duration);
      }

      const info = await invoke("get_media_info");
      if (info) {
        const [title, coverArt] = info as [string, string | undefined];
        setMediaTitle(title);
        setMediaCoverArt(coverArt);
      }

    }, 1000);

    return () => {
      unlistenVolume.then((fn) => fn());
      clearInterval(interval);
    };
  }, []);

  async function handleVolumeChange(newVolume: number) {
    await invoke("set_volume", { volume: newVolume });
  }

  async function handleSeek(offset: number) {
    await invoke("seek", { offset });
  }

  async function handleSetMediaPosition(newPosition: number) {
    await invoke("set_position", { position: newPosition });
  }

  async function handleNextTrack() {
    await invoke("next_track");
  }

  async function handlePreviousTrack() {
    await invoke("previous_track");
  }

  const formatTime = (seconds: number) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    return `${minutes}:${remainingSeconds < 10 ? '0' : ''}${remainingSeconds}`;
  };

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

      <h2>Media Controls</h2>
      <div className="row">
        <button onClick={() => handleSeek(-10)}>Back 10s</button>
        <button onClick={() => handleSeek(10)}>Forward 10s</button>
        <button onClick={handlePreviousTrack}>Previous</button>
        <button onClick={handleNextTrack}>Next</button>
      </div>

      {mediaTitle && <p>Now Playing: {mediaTitle}</p>}
      {mediaCoverArt && <img src={mediaCoverArt} alt="Media Cover" style={{ width: '100px', height: '100px' }} />}

      {mediaDuration > 0 && (
        <div className="row">
          <input
            type="range"
            min="0"
            max={mediaDuration}
            value={mediaPosition}
            onChange={(e) => handleSetMediaPosition(parseFloat(e.currentTarget.value))}
          />
          <span>{formatTime(mediaPosition)} / {formatTime(mediaDuration)}</span>
        </div>
      )}
    </main>
  );
}

export default App;
