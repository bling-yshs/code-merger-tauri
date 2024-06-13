import json

import requests
from packaging import version


def get_latest_release_tag(owner, repo):
    url = f"https://api.github.com/repos/{owner}/{repo}/releases/latest"
    response = requests.get(url)

    if response.status_code == 200:
        latest_release = response.json()
        return latest_release['tag_name']
    else:
        return "v9.9.9"


def compare_version(current_version, latest_version):
    return version.parse(current_version) > version.parse(latest_version)


def get_current_version():
    with open('./src-tauri/tauri.conf.json', 'r', encoding='utf-8') as file:
        data = json.load(file)
    return data['version']


if __name__ == "__main__":
    # 需要发布新版本输出新的版本号，否则返回 False
    owner_name = "bling-yshs"
    repo_name = "code-merger-tauri"
    latest = get_latest_release_tag(owner_name, repo_name)
    current = get_current_version()
    if compare_version(current, latest):
        print(f'v{current}')
    else:
        print(False)
