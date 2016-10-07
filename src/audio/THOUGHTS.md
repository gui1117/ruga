port baal to rodio instead of portaudio change things. So here is thoughts on
the new design

we use a king of pipeline on sources :

```
sink <- global [play, pause, set_volume] <- music [play, play_or_continue, pause, resume, stop, set_volume, set_transitions]
                                         <- effects [set_volume, set_listener] <- short [play, stop_all]
                                                                               <- persistent [pause, resume, mute, unmute, compute_volumes,
                                                                                              add_positions(vec), clear_positions(id)]
```
