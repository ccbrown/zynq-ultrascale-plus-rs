#!/usr/bin/env python3
import concurrent.futures
import json
import os
import requests
from bs4 import BeautifulSoup

SUMMARY_URL = 'https://www.xilinx.com/htmldocs/registers/ug1087/_module_summary.html'

out_path = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'modules.json')
if os.path.isfile(out_path):
    os.unlink(out_path)

print('fetching module summary')


def http_get(url):
    failures = 0
    while True:
        try:
            r = requests.get(url)
            r.raise_for_status()
            if failures > 0:
                print(f'request succeeded after {failures} failures')
            return r.text
        except Exception as e:
            print(f'http error: {e}. trying again...')
            failures += 1


summaries = {}

soup = BeautifulSoup(http_get(SUMMARY_URL), 'html.parser')
for row in soup.find_all('tr'):
    cells = row.find_all('td')
    if len(cells) > 0:
        module_name = cells[0].text
        module_type = cells[1].find('a').text
        docs_path = cells[1].find('a').get('onclick').split('"')[1]
        base_address = int(cells[2].text, base=16)
        description = cells[3].text
        summaries[module_name] = {
            'type': module_type,
            'docs_path': docs_path,
            'base_address': base_address,
            'description': description,
        }

print(f'scraped {len(summaries)} module summaries')


def scrape_fields(docs_path):
    fields = []
    url = 'https://www.xilinx.com/htmldocs/registers/ug1087/' + docs_path
    soup = BeautifulSoup(http_get(url), 'html.parser')
    for row in soup.find_all('tr'):
        cells = row.find_all('td')
        if len(cells) >= 5:
            fields.append({
                'name': cells[0].text,
                'bits': cells[1].text,
                'type': cells[2].find(text=True, recursive=False),
                'description': cells[4].text,
            })
    return fields


def scrape_module_type_registers(docs_path):
    print(f'scraping registers from {docs_path}')

    url = 'https://www.xilinx.com/htmldocs/registers/ug1087/' + docs_path
    soup = BeautifulSoup(http_get(url), 'html.parser')

    registers = []
    for row in soup.find_all('tr'):
        cells = row.find_all('td')
        if len(cells) >= 6:
            docs_path = cells[0].find('a').get('onclick').split('"')[1]
            fields = scrape_fields(docs_path)
            registers.append({
                'name': cells[0].text,
                'address': int(cells[1].text, base=16),
                'width': int(cells[2].text.strip()),
                'type': cells[3].find(text=True, recursive=False),
                'description': cells[5].text,
                'fields': fields,
            })
    return registers


out = {}

with concurrent.futures.ThreadPoolExecutor(max_workers=200) as executor:
    for module_name, summary in summaries.items():
        if summary['type'] not in out:
            out[summary['type']] = {
                'registers': executor.submit(scrape_module_type_registers, summary['docs_path']),
                'modules': {},
            }
        out[summary['type']]['modules'][module_name] = {
            'base_address': summary['base_address'],
            'description': summary['description'],
        }

    for module_type_name, v in out.items():
        print('waiting for ' + module_type_name)
        v['registers'] = v['registers'].result()

with open(out_path, 'w') as f:
    json.dump(out, f, indent=4)
