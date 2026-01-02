#set tvc as a valid 'CEC participant'
sudo cec-ctl --playback --osd-name "tvc"

#power off
sudo cec-ctl --to 0 --standby


#power on
sudo cec-ctl --to 0 --user-control-pressed ui-cmd=power
sudo cec-ctl --to 0 --user-control-released


#volume up
sudo cec-ctl --to 0 --user-control-pressed ui-cmd=volume-up
sudo cec-ctl --to 0 --user-control-released


#volume down
sudo cec-ctl --to 0 --user-control-pressed ui-cmd=volume-down
sudo cec-ctl --to 0 --user-control-released



sudo cec-ctl --active-source phys-addr=1.0.0.0
