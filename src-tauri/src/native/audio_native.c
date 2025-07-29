#include "audio_native.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Static variables for performance optimization
static audio_control_t global_audio_ctrl = {0};
static int audio_initialized = 0;

// Fast ALSA implementation with minimal overhead
static int alsa_init(audio_control_t* ctrl) {
    snd_mixer_t** handle = (snd_mixer_t**)&ctrl->handle;
    
    if (snd_mixer_open(handle, 0) < 0) return -1;
    if (snd_mixer_attach(*handle, "default") < 0) goto error;
    if (snd_mixer_selem_register(*handle, NULL, NULL) < 0) goto error;
    if (snd_mixer_load(*handle) < 0) goto error;
    
    return 0;
    
error:
    snd_mixer_close(*handle);
    return -1;
}

static void alsa_cleanup(snd_mixer_t* handle) {
    if (handle) {
        snd_mixer_close(handle);
    }
}

static long alsa_get_volume(snd_mixer_t* handle) {
    snd_mixer_selem_id_t *sid;
    snd_mixer_elem_t* elem;
    long volume = 0, min, max;
    
    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, "Master");
    
    elem = snd_mixer_find_selem(handle, sid);
    if (!elem) return 0;
    
    snd_mixer_selem_get_playback_volume_range(elem, &min, &max);
    snd_mixer_selem_get_playback_volume(elem, SND_MIXER_SCHN_FRONT_LEFT, &volume);
    
    return (100 * (volume - min)) / (max - min);
}

static int alsa_set_volume(snd_mixer_t* handle, long volume) {
    snd_mixer_selem_id_t *sid;
    snd_mixer_elem_t* elem;
    long min, max;
    
    snd_mixer_selem_id_alloca(&sid);
    snd_mixer_selem_id_set_index(sid, 0);
    snd_mixer_selem_id_set_name(sid, "Master");
    
    elem = snd_mixer_find_selem(handle, sid);
    if (!elem) return -1;
    
    snd_mixer_selem_get_playback_volume_range(elem, &min, &max);
    long target = min + (volume * (max - min)) / 100;
    
    return snd_mixer_selem_set_playback_volume_all(elem, target);
}

// Public API with automatic backend detection and caching
int audio_init(audio_control_t* ctrl) {
    if (audio_initialized && global_audio_ctrl.handle) {
        *ctrl = global_audio_ctrl;
        return 0;
    }
    
    // Try ALSA first (most common and fastest)
    ctrl->backend = AUDIO_BACKEND_ALSA;
    if (alsa_init(ctrl) == 0) {
        global_audio_ctrl = *ctrl;
        audio_initialized = 1;
        return 0;
    }
    
    return -1;
}

void audio_cleanup(audio_control_t* ctrl) {
    if (!ctrl || !ctrl->handle) return;
    
    switch (ctrl->backend) {
        case AUDIO_BACKEND_ALSA:
            alsa_cleanup((snd_mixer_t*)ctrl->handle);
            break;
        default:
            break;
    }
    
    ctrl->handle = NULL;
    audio_initialized = 0;
}

long audio_get_volume(audio_control_t* ctrl) {
    if (!ctrl || !ctrl->handle) return 0;
    
    switch (ctrl->backend) {
        case AUDIO_BACKEND_ALSA:
            return alsa_get_volume((snd_mixer_t*)ctrl->handle);
        default:
            return 0;
    }
}

int audio_set_volume(audio_control_t* ctrl, long volume) {
    if (!ctrl || !ctrl->handle) return -1;
    if (volume < 0) volume = 0;
    if (volume > 100) volume = 100;
    
    switch (ctrl->backend) {
        case AUDIO_BACKEND_ALSA:
            return alsa_set_volume((snd_mixer_t*)ctrl->handle, volume);
        default:
            return -1;
    }
}

int audio_get_mute(audio_control_t* ctrl) {
    if (!ctrl || !ctrl->handle) return 0;
    
    // Implementation for mute detection
    snd_mixer_selem_id_t *sid;
    snd_mixer_elem_t* elem;
    int mute = 0;
    
    if (ctrl->backend == AUDIO_BACKEND_ALSA) {
        snd_mixer_t* handle = (snd_mixer_t*)ctrl->handle;
        snd_mixer_selem_id_alloca(&sid);
        snd_mixer_selem_id_set_index(sid, 0);
        snd_mixer_selem_id_set_name(sid, "Master");
        
        elem = snd_mixer_find_selem(handle, sid);
        if (elem) {
            snd_mixer_selem_get_playback_switch(elem, SND_MIXER_SCHN_FRONT_LEFT, &mute);
            return !mute; // ALSA returns 1 for unmuted, we want 1 for muted
        }
    }
    
    return 0;
}

int audio_set_mute(audio_control_t* ctrl, int mute) {
    if (!ctrl || !ctrl->handle) return -1;
    
    snd_mixer_selem_id_t *sid;
    snd_mixer_elem_t* elem;
    
    if (ctrl->backend == AUDIO_BACKEND_ALSA) {
        snd_mixer_t* handle = (snd_mixer_t*)ctrl->handle;
        snd_mixer_selem_id_alloca(&sid);
        snd_mixer_selem_id_set_index(sid, 0);
        snd_mixer_selem_id_set_name(sid, "Master");
        
        elem = snd_mixer_find_selem(handle, sid);
        if (elem) {
            return snd_mixer_selem_set_playback_switch_all(elem, !mute);
        }
    }
    
    return -1;
}
