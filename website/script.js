function getOs() {
    let agent = navigator.userAgent;
    if (agent.indexOf("Win") !== -1)
        return "windows";
    else if (agent.indexOf("Mac") !== -1)
        return "mac";
    else if (agent.indexOf("X11") !== -1 || agent.indexOf("Linux") !== -1)
        return "linux";
    else
        return "unknown";
}

function selectTab(selector, tabs, tabInfos) {
    const selected = document.querySelector(selector);
    tabs.forEach(tabInfo => {
        tabInfo.classList.remove('active');
    })
    tabInfos.forEach(tabInfo => {
        tabInfo.classList.remove('active');
    })
    selected.classList.add('active');
    let id = selected.id;
    document.querySelector(`[data-tab-value="#${id}"]`).classList.add('active');
}

function setUpTabs() {
    const tabs = document.querySelectorAll('[data-tab-value]');
    const tabInfos = document.querySelectorAll('[data-tab-info]');
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            selectTab(tab.dataset.tabValue, tabs, tabInfos);
        });
    })

    const os = getOs();
    if (os === "windows")
        selectTab('#tab_windows', tabs, tabInfos);
    else if (os === "mac")
        selectTab('#tab_macos', tabs, tabInfos);
    else
        selectTab('#tab_linux', tabs, tabInfos);
}
