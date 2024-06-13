import json


def main():
    # 读取 ./src-tauri/tauri.conf.json 文件，将 version 字段的值修改为 9.9.9
    with open('./src-tauri/tauri.conf.json', 'r', encoding='utf-8') as file:
        data = json.load(file)

    data['version'] = '9.9.9'

    with open('./src-tauri/tauri.conf.json', 'w', encoding='utf-8') as file:
        json.dump(data, file, ensure_ascii=False, indent=2)


if __name__ == "__main__":
    main()
