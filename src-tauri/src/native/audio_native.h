#ifndef AUDIO_NATIVE_H
#define AUDIO_NATIVE_H

#include <alsa/asoundlib.h>
#include <sys/ioctl.h>
#include <fcntl.h>
#include <unistd.h>

// Audio system backends
typedef enum {
    AUDIO_BACKEND_ALSA = 0,
    AUDIO_BACKEND_OSS = 1
} audio_backend_t;

// Audio control structure
typedef struct {
    audio_backend_t backend;
    void* handle;
    int fd;
} audio_control_t;

// Function prototypes
int audio_init(audio_control_t* ctrl);
void audio_cleanup(audio_control_t* ctrl);
long audio_get_volume(audio_control_t* ctrl);
int audio_set_volume(audio_control_t* ctrl, long volume);
int audio_get_mute(audio_control_t* ctrl);
int audio_set_mute(audio_control_t* ctrl, int mute);

#endif // AUDIO_NATIVE_H
