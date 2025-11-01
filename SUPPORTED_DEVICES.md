
# Supported devices

&check; - Rust only\
&#x2705; - Rust and Python

| Feature<br/><br/><br/>               | GenericDevice<br/><br/><br/> | L510<br/>L520<br/>L610<br/> | L530<br/>L535<br/>L630<br/> | L900<br/><br/><br/> | L920<br/>L930<br/><br/> | P100<br/>P105<br/><br/> | P110<br/>P110M<br/>P115<br/> | P300<br/>P306<br/><br/> | P304M<br/>P316M<br/><br/> | H100<br/><br/><br/> |
| ------------------------------------ | :--------------------------- | :-------------------------- | :-------------------------- | :------------------ | :---------------------- | :---------------------- | :--------------------------- | :---------------------- | :------------------------ | :------------------ |
| device_reboot                        |                              | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     | &#x2705;                | &#x2705;                  | &#x2705;            |
| device_reset                         |                              | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_child_device_component_list_json |                              |                             |                             |                     |                         |                         |                              | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_child_device_list                |                              |                             |                             |                     |                         |                         |                              | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_child_device_list_json           |                              |                             |                             |                     |                         |                         |                              | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_current_power                    |                              |                             |                             |                     |                         |                         | &#x2705;                     |                         |                           |                     |
| get_device_info                      | &#x2705;                     | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_device_info_json                 | &#x2705;                     | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     | &#x2705;                | &#x2705;                  | &#x2705;            |
| get_device_usage                     |                              | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     |                         |                           |                     |
| get_energy_data                      |                              |                             |                             |                     |                         |                         | &#x2705;                     |                         |                           |                     |
| get_energy_usage                     |                              |                             |                             |                     |                         |                         | &#x2705;                     |                         |                           |                     |
| get_power_data                       |                              |                             |                             |                     |                         |                         | &#x2705;                     |                         |                           |                     |
| get_supported_ringtone_list          |                              |                             |                             |                     |                         |                         |                              |                         |                           | &#x2705;            |
| off                                  | &#x2705;                     | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     |                         |                           |                     |
| on                                   | &#x2705;                     | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     |                         |                           |                     |
| play_alarm                           |                              |                             |                             |                     |                         |                         |                              |                         |                           | &#x2705;            |
| refresh_session                      | &#x2705;                     | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                | &#x2705;                | &#x2705;                     | &#x2705;                | &#x2705;                  | &#x2705;            |
| set_brightness                       |                              | &#x2705;                    | &#x2705;                    | &#x2705;            | &#x2705;                |                         |                              |                         |                           |                     |
| set_color                            |                              |                             | &#x2705;                    | &#x2705;            | &#x2705;                |                         |                              |                         |                           |                     |
| set_color_temperature                |                              |                             | &#x2705;                    | &#x2705;            | &#x2705;                |                         |                              |                         |                           |                     |
| set_hue_saturation                   |                              |                             | &#x2705;                    | &#x2705;            | &#x2705;                |                         |                              |                         |                           |                     |
| set_lighting_effect                  |                              |                             |                             |                     | &#x2705;                |                         |                              |                         |                           |                     |
| set() API \*                         |                              |                             | &#x2705;                    | &#x2705;            | &#x2705;                |                         |                              |                         |                           |                     |
| stop_alarm                           |                              |                             |                             |                     |                         |                         |                              |                         |                           | &#x2705;            |


\* The `set()` API allows multiple properties to be set in a single request.

## Hub (H100) Child Devices

&check; - Rust only\
&#x2705; - Rust and Python

| Feature<br/><br/>                | KE100<br/><br/> | S200B<br/><br/> | T100<br/><br/> | T110<br/><br/> | T300<br/><br/> | T310<br/>T315 |
| -------------------------------- | :-------------- | :-------------- | :------------- | :------------- | :------------- | :------------ |
| get_device_info \*               | &#x2705;        | &#x2705;        | &#x2705;       | &#x2705;       | &#x2705;       | &#x2705;      |
| get_device_info_json             | &#x2705;        | &#x2705;        | &#x2705;       | &#x2705;       | &#x2705;       | &#x2705;      |
| get_temperature_humidity_records |                 |                 |                |                |                | &#x2705;      |
| get_trigger_logs                 |                 | &#x2705;        | &#x2705;       | &#x2705;       | &#x2705;       |               |
| set_child_protection             | &#x2705;        |                 |                |                |                |               |
| set_frost_protection             | &#x2705;        |                 |                |                |                |               |
| set_max_control_temperature      | &#x2705;        |                 |                |                |                |               |
| set_min_control_temperature      | &#x2705;        |                 |                |                |                |               |
| set_target_temperature           | &#x2705;        |                 |                |                |                |               |
| set_temperature_offset           | &#x2705;        |                 |                |                |                |               |

\* Obtained by calling `get_child_device_list` on the hub device or `get_device_info` on a child device handler.

## Power Strips Child Devices

&check; - Rust only\
&#x2705; - Rust and Python

| Feature<br/><br/>    | P300<br/>P306<br/> | P304M<br/>P316M<br/> |
| -------------------- | :----------------- | :------------------- |
| get_current_power    |                    | &#x2705;             |
| get_device_info \*   | &#x2705;           | &#x2705;             |
| get_device_info_json | &#x2705;           | &#x2705;             |
| get_device_usage     |                    | &#x2705;             |
| get_energy_data      |                    | &#x2705;             |
| get_energy_usage     |                    | &#x2705;             |
| get_power_data       |                    | &#x2705;             |
| off                  | &#x2705;           | &#x2705;             |
| on                   | &#x2705;           | &#x2705;             |

\* Obtained by calling `get_child_device_list` on the hub device or `get_device_info` on a child device handler.
