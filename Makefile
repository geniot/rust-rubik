PROJECT_NAME := rust-rubik
PROGRAM_NAME := rust-rubik
DEPLOY_PATH := /mnt/SDCARD/Apps/Snake

IP := ${IP}
USN := root
PWD := tina

all: clean docker deploy

dist: clean docker

clean:
	rm dist -rf

docker:
	#docker run -d --name trimui-sdk -c 1024 -it --volume=/opt/TrimuiProjects/:/work/ --workdir=/work/ trimui-sdk
	docker exec trimui-sdk /bin/bash -c 'cd ${PROJECT_NAME} && make build'

build:
	rustup target add aarch64-unknown-linux-gnu && \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-g++" \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=--sysroot=${SYSROOT}" \
    CC_ENABLE_DEBUG_OUTPUT=1 \
    cargo build -vv --target=aarch64-unknown-linux-gnu --target-dir bin --bin ${PROGRAM_NAME}


deploy:
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "rm ${DEPLOY_PATH}/${PROGRAM_NAME} -f"
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "rm ${DEPLOY_PATH}/libraylib.so -f"
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "rm ${DEPLOY_PATH}/libraylib.so.600 -f"
	sshpass -p ${PWD} scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null lib/arm64/libraylib.so ${USN}@${IP}:${DEPLOY_PATH}/libraylib.so.600
	sshpass -p ${PWD} scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null bin/aarch64-unknown-linux-gnu/debug/${PROGRAM_NAME} ${USN}@${IP}:${DEPLOY_PATH}
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "chmod 777 ${DEPLOY_PATH}/${PROGRAM_NAME}"
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "if pgrep ${PROGRAM_NAME}; then pkill -f ${PROGRAM_NAME}; fi"
	sshpass -p ${PWD} ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null ${USN}@${IP} "sh -c 'cd /tmp; LD_LIBRARY_PATH=${DEPLOY_PATH}:$LD_LIBRARY_PATH ${DEPLOY_PATH}/${PROGRAM_NAME}'" &

bindgen:
	bindgen lib/raylib.h -o src/bindings.rs