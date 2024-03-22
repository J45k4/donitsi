FROM eu.gcr.io/pt-container/unicorn:arm32v7-rust
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
WORKDIR /src/fontconfig
# RUN wget https://www.freedesktop.org/software/fontconfig/release/fontconfig-2.11.1.tar.gz
# RUN wget https://www.freedesktop.org/software/fontconfig/release/fontconfig-2.11.1.tar.gz
# RUN curl -O https://www.freedesktop.org/software/fontconfig/release/fontconfig-2.11.1.tar.gz
RUN curl -k -O https://www.freedesktop.org/software/fontconfig/release/fontconfig-2.11.1.tar.gz


RUN tar -xvf fontconfig-2.11.1.tar.gz
WORKDIR /src/fontconfig/fontconfig-2.11.1
RUN ./configure
RUN make
RUN make install
# RUN fc-cache -fv
WORKDIR /src/donitsi
COPY src /src/donitsi/src
COPY Cargo.toml /src/donitsi/Cargo.toml
COPY Cargo.lock /src/donitsi/Cargo.lock
COPY ./shaders /src/donitsi/shaders
RUN ~/.cargo/bin/cargo build  --release
RUN sed -i s/deb.debian.org/archive.debian.org/g /etc/apt/sources.list
RUN sed -i 's|security.debian.org|archive.debian.org/|g' /etc/apt/sources.list
RUN sed -i '/stretch-updates/d' /etc/apt/sources.list
RUN echo "deb http://archive.debian.org/debian stretch-backports main" > /etc/apt/sources.list.d/backports.list
RUN apt-get update

RUN apt install -y libxcursor1
RUN apt install -y libxrandr2