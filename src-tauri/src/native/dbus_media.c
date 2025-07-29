#include <dbus/dbus.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

// Media control structure
typedef struct {
    DBusConnection* connection;
    char* service_name;
    int is_connected;
} media_control_t;

static media_control_t global_media_ctrl = {0};

// MPRIS D-Bus interface constants
#define MPRIS_PREFIX "org.mpris.MediaPlayer2"
#define MPRIS_OBJECT_PATH "/org/mpris/MediaPlayer2"
#define MPRIS_PLAYER_INTERFACE "org.mpris.MediaPlayer2.Player"
#define DBUS_PROPERTIES_INTERFACE "org.freedesktop.DBus.Properties"

// Fast D-Bus connection with minimal overhead
int media_init() {
    if (global_media_ctrl.is_connected) {
        return 0;
    }
    
    DBusError error;
    dbus_error_init(&error);
    
    global_media_ctrl.connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
    if (dbus_error_is_set(&error)) {
        dbus_error_free(&error);
        return -1;
    }
    
    // Find active media player
    DBusMessage* msg = dbus_message_new_method_call(
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
        "ListNames"
    );
    
    DBusMessage* reply = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 1000, &error
    );
    
    if (reply) {
        DBusMessageIter iter, sub_iter;
        dbus_message_iter_init(reply, &iter);
        dbus_message_iter_recurse(&iter, &sub_iter);
        
        while (dbus_message_iter_get_arg_type(&sub_iter) == DBUS_TYPE_STRING) {
            const char* name;
            dbus_message_iter_get_basic(&sub_iter, &name);
            
            if (strncmp(name, MPRIS_PREFIX, strlen(MPRIS_PREFIX)) == 0) {
                global_media_ctrl.service_name = strdup(name);
                global_media_ctrl.is_connected = 1;
                break;
            }
            
            dbus_message_iter_next(&sub_iter);
        }
        
        dbus_message_unref(reply);
    }
    
    dbus_message_unref(msg);
    dbus_error_free(&error);
    
    return global_media_ctrl.is_connected ? 0 : -1;
}

void media_cleanup() {
    if (global_media_ctrl.service_name) {
        free(global_media_ctrl.service_name);
        global_media_ctrl.service_name = NULL;
    }
    
    if (global_media_ctrl.connection) {
        dbus_connection_unref(global_media_ctrl.connection);
        global_media_ctrl.connection = NULL;
    }
    
    global_media_ctrl.is_connected = 0;
}

// High-performance D-Bus method call
static int call_media_method(const char* method) {
    if (!global_media_ctrl.is_connected) {
        if (media_init() != 0) return -1;
    }
    
    DBusMessage* msg = dbus_message_new_method_call(
        global_media_ctrl.service_name,
        MPRIS_OBJECT_PATH,
        MPRIS_PLAYER_INTERFACE,
        method
    );
    
    if (!msg) return -1;
    
    DBusMessage* reply = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 500, NULL
    );
    
    dbus_message_unref(msg);
    if (reply) {
        dbus_message_unref(reply);
        return 0;
    }
    
    return -1;
}

// Media control functions
int media_play() {
    return call_media_method("Play");
}

int media_pause() {
    return call_media_method("Pause");
}

int media_next() {
    return call_media_method("Next");
}

int media_previous() {
    return call_media_method("Previous");
}

int media_seek(long offset_microseconds) {
    if (!global_media_ctrl.is_connected) {
        if (media_init() != 0) return -1;
    }
    
    DBusMessage* msg = dbus_message_new_method_call(
        global_media_ctrl.service_name,
        MPRIS_OBJECT_PATH,
        MPRIS_PLAYER_INTERFACE,
        "Seek"
    );
    
    if (!msg) return -1;
    
    dbus_int64_t offset = offset_microseconds;
    dbus_message_append_args(msg, DBUS_TYPE_INT64, &offset, DBUS_TYPE_INVALID);
    
    DBusMessage* reply = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 500, NULL
    );
    
    dbus_message_unref(msg);
    if (reply) {
        dbus_message_unref(reply);
        return 0;
    }
    
    return -1;
}

// Get media position (returns microseconds)
long media_get_position() {
    if (!global_media_ctrl.is_connected) {
        if (media_init() != 0) return -1;
    }
    
    DBusMessage* msg = dbus_message_new_method_call(
        global_media_ctrl.service_name,
        MPRIS_OBJECT_PATH,
        DBUS_PROPERTIES_INTERFACE,
        "Get"
    );
    
    if (!msg) return -1;
    
    const char* interface = MPRIS_PLAYER_INTERFACE;
    const char* property = "Position";
    
    dbus_message_append_args(msg,
        DBUS_TYPE_STRING, &interface,
        DBUS_TYPE_STRING, &property,
        DBUS_TYPE_INVALID
    );
    
    DBusMessage* reply = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 500, NULL
    );
    
    dbus_message_unref(msg);
    
    if (reply) {
        DBusMessageIter iter, variant_iter;
        dbus_message_iter_init(reply, &iter);
        dbus_message_iter_recurse(&iter, &variant_iter);
        
        dbus_int64_t position;
        dbus_message_iter_get_basic(&variant_iter, &position);
        dbus_message_unref(reply);
        
        return (long)position;
    }
    
    return -1;
}

// Set media position (microseconds)
int media_set_position(long position_microseconds) {
    if (!global_media_ctrl.is_connected) {
        if (media_init() != 0) return -1;
    }
    
    // First get track ID
    DBusMessage* msg = dbus_message_new_method_call(
        global_media_ctrl.service_name,
        MPRIS_OBJECT_PATH,
        DBUS_PROPERTIES_INTERFACE,
        "Get"
    );
    
    if (!msg) return -1;
    
    const char* interface = MPRIS_PLAYER_INTERFACE;
    const char* property = "Metadata";
    
    dbus_message_append_args(msg,
        DBUS_TYPE_STRING, &interface,
        DBUS_TYPE_STRING, &property,
        DBUS_TYPE_INVALID
    );
    
    DBusMessage* reply = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 500, NULL
    );
    
    dbus_message_unref(msg);
    
    if (!reply) return -1;
    
    // Extract track ID from metadata (simplified)
    char* track_id = "/some/track/id"; // This would need proper parsing
    
    // Now set position
    msg = dbus_message_new_method_call(
        global_media_ctrl.service_name,
        MPRIS_OBJECT_PATH,
        MPRIS_PLAYER_INTERFACE,
        "SetPosition"
    );
    
    dbus_int64_t position = position_microseconds;
    dbus_message_append_args(msg,
        DBUS_TYPE_OBJECT_PATH, &track_id,
        DBUS_TYPE_INT64, &position,
        DBUS_TYPE_INVALID
    );
    
    DBusMessage* reply2 = dbus_connection_send_with_reply_and_block(
        global_media_ctrl.connection, msg, 500, NULL
    );
    
    dbus_message_unref(msg);
    dbus_message_unref(reply);
    
    if (reply2) {
        dbus_message_unref(reply2);
        return 0;
    }
    
    return -1;
}
