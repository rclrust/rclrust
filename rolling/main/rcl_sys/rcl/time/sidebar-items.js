initSidebarItems({"enum":[["RclClockType","Time source type, used to indicate the source of a time measurement."],["rcl_clock_change_t","Enumeration to describe the type of time jump."]],"fn":[["rcl_clock_add_jump_callback","Add a callback to be called when a time jump exceeds a threshold."],["rcl_clock_fini","Finalize a clock."],["rcl_clock_get_now","Fill the time point value with the current value of the associated clock."],["rcl_clock_init","Initialize a clock based on the passed type."],["rcl_clock_remove_jump_callback","Remove a previously added time jump callback."],["rcl_clock_valid","Check if the clock has valid values."],["rcl_difference_times","Compute the difference between two time points"],["rcl_disable_ros_time_override","Disable the ROS time abstraction override."],["rcl_enable_ros_time_override","Enable the ROS time abstraction override."],["rcl_is_enabled_ros_time_override","Check if the `RCL_ROS_TIME` time source has the override enabled."],["rcl_ros_clock_fini","Finalize a clock as a `RCL_ROS_TIME` time source."],["rcl_ros_clock_init","Initialize a clock as a RCL_ROS_TIME time source."],["rcl_set_ros_time_override","Set the current time for this `RCL_ROS_TIME` time source."],["rcl_steady_clock_fini","Finalize a clock as a `RCL_STEADY_TIME` time source."],["rcl_steady_clock_init","Initialize a clock as a `RCL_STEADY_TIME` time source."],["rcl_system_clock_fini","Finalize a clock as a `RCL_SYSTEM_TIME` time source."],["rcl_system_clock_init","Initialize a clock as a `RCL_SYSTEM_TIME` time source."]],"struct":[["rcl_clock_t","Encapsulation of a time source."],["rcl_duration_t","A duration of time, measured in nanoseconds and its source."],["rcl_jump_callback_info_t","Struct to describe an added callback."],["rcl_jump_threshold_t","Describe the prerequisites for calling a time jump callback."],["rcl_time_jump_t","Struct to describe a jump in time."],["rcl_time_point_t","A single point in time, measured in nanoseconds, the reference point is based on the source."]],"type":[["rcl_duration_value_t","A duration of time, measured in nanoseconds."],["rcl_jump_callback_t","Signature of a time jump callback."],["rcl_time_point_value_t","A single point in time, measured in nanoseconds since the Unix epoch."]]});