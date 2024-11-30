conf_dir := '$HOME/.config/poweroff-rs'

install:
    # install the binary
    cargo install --path .
    
    # install the icon
    sudo just icons/install

defaultconf:
    # create default config
    mkdir -p {{conf_dir}}
    cp config/config.ron {{conf_dir}}/
    cp config/style.css {{conf_dir}}/

uninstall:
    # remove config
    echo Not removing config at {{conf_dir}}/config.ron

    # removing binary
    cargo uninstall

    # removing icons
    sudo just icons/uninstall
