KEYBOARD="DC:2C:26:00:11:EF"
DONGLE="5C:F3:70:A8:B2:9F"

#target/release/bt disconnect "${DONGLE}" ${KEYBOARD}

target/release/bt connect "${DONGLE}" ${KEYBOARD} 