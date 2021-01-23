#include <gdk/gdk.h>
#include <gtk/gtk.h>
#include <stdlib.h>
#include <time.h>

typedef struct _key {
  guint16 hardward;
  guint32 keyval;
  guint32 state;
} key;

static clock_t start;

static key keys[] = {{53, GDK_KEY_x, 0},
                     {33, GDK_KEY_p, 0},
                     {
                         28,
                         GDK_KEY_t,
                         0,
                     },
                     {
                         58,
                         GDK_KEY_m,
                         0,
                     },
                     {
                         53,
                         GDK_KEY_x,
                         0,
                     },
                     {58, GDK_KEY_m, 0}};

void send_event(GdkEventKey *kev, key key) {
  kev->send_event = TRUE;
  kev->hardware_keycode = 100;
  kev->keyval = GDK_KEY_Henkan_Mode;
  kev->state = 0;
  gdk_event_put((GdkEvent *)kev);
}

gboolean key_press(GtkWidget *text_view, GdkEventKey *get_ev) {
  if (get_ev->send_event) {
    if (get_ev->keyval == GDK_KEY_Return) {
      clock_t end = clock();
      printf("%ld clocks\n", (end - start));

      GtkTextBuffer *buf = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_view));
      GtkTextIter start_iter, end_iter;
      gtk_text_buffer_get_start_iter(buf, &start_iter);
      gtk_text_buffer_get_end_iter(buf, &end_iter);
      gchar* text = gtk_text_buffer_get_text(buf, &start_iter, &end_iter, TRUE);

      printf("%s\n", text);

      fflush(stdout);
      exit(0);
    }
    return FALSE;
  }

  if (get_ev->keyval == GDK_KEY_Return) {
    GdkEvent *ev = gdk_event_new(GDK_KEY_PRESS);

    ev->key.window = gtk_widget_get_parent_window(text_view);

    // enable hangul
    {
      GdkEventKey *kev = &ev->key;
      kev->send_event = TRUE;
      kev->hardware_keycode = 100;
      kev->keyval = GDK_KEY_Henkan_Mode;
      kev->state = 0;
      gdk_event_put(ev);
    }

    for (int set = 0; set < 40; set++) {
      for (int i = 0; i < G_N_ELEMENTS(keys); i++) {
        GdkEventKey *kev = &ev->key;
        kev->send_event = TRUE;
        kev->hardware_keycode = keys[i].hardward;
        kev->keyval = keys[i].keyval;
        kev->state = keys[i].state;
        kev->window = gtk_widget_get_parent_window(text_view);
        gdk_event_put(ev);
      }
    }
    
    // esc
    {
      GdkEventKey *kev = &ev->key;
      kev->send_event = TRUE;
      kev->hardware_keycode = 9;
      kev->keyval = GDK_KEY_Escape;
      kev->state = 0;
      gdk_event_put(ev);
    }
    
    // enter
    {
      GdkEventKey *kev = &ev->key;
      kev->send_event = TRUE;
      kev->hardware_keycode = 36;
      kev->keyval = GDK_KEY_Return;
      kev->state = 0;
      gdk_event_put(ev);
    }

    gdk_event_free(ev);
    start = clock();
    return TRUE;
  }

  return FALSE;
}

void activate(GtkApplication *app, gpointer user_data) {
  GtkWidget *window = gtk_application_window_new(GTK_APPLICATION(app));
  gtk_window_set_title(GTK_WINDOW(window), "Bench");
  GtkWidget *text_view = gtk_text_view_new();
  gtk_container_add(GTK_CONTAINER(window), text_view);
  gtk_widget_add_events(text_view, GDK_KEY_PRESS_MASK);

  gtk_widget_show_all(window);

  g_signal_connect(text_view, "key-press-event", G_CALLBACK(key_press), NULL);
}

int main(int argc, char **argv) {
  GtkApplication *app = gtk_application_new("github.riey.korean-ime-benchmark",
                                            G_APPLICATION_FLAGS_NONE);
  g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
  int status = g_application_run(G_APPLICATION(app), argc, argv);
  g_object_unref(app);
  return status;
}
