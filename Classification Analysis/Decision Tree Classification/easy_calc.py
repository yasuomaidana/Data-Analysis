worked_time = (4, 19)
used_time = (2, 41)

remaining_time = (worked_time[0] - used_time[0], worked_time[1] - used_time[1])
if remaining_time[1] < 0:
    remaining_time = (remaining_time[0] - 1, remaining_time[1] + 60)
print(remaining_time)