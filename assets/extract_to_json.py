from html.parser import HTMLParser
import json


class TableMiningParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.n_tbody = 0
        self.content_list = []
        self.content_row = []
        self.in_header_row = False
        self.in_tr = False
        self.in_td = False
        self.analyze_finished = False
        self.in_link_in_td = False

    def handle_starttag(self, tag, attrs):
        if self.analyze_finished:
            return
        if tag == "tbody":
            self.n_tbody += 1
        if self.n_tbody == 1:
            if tag == "tr":
                self.in_tr = True
            elif self.in_tr:
                if tag == "th":
                    self.in_header_row = True
                elif tag == "td":
                    self.in_td = True
                elif self.in_td and tag == "a":
                    self.in_link_in_td = True

    def handle_endtag(self, tag):
        if self.analyze_finished:
            return
        if tag == "tbody":
            self.n_tbody -= 1
            if self.n_tbody == 0:
                self.analyze_finished = True
        if self.n_tbody == 1:
            if self.in_tr:
                if self.in_td:
                    if tag == "td":
                        self.in_td = False
                    elif self.in_link_in_td and tag == "a":
                        self.in_link_in_td = False
                elif tag == "tr":
                    self.in_tr = False
                    if not self.in_header_row:
                        self.content_list.append(self.content_row)
                        self.content_row = []
                    self.in_header_row = False

    def handle_data(self, data):
        if self.n_tbody == 1 and self.in_tr and self.in_td:
            cleaned_data = data.strip().strip("\u200E\u200F")
            if self.in_link_in_td or cleaned_data != "":
                self.content_row.append(cleaned_data)


def convert_row(row):
    (
        locale_id_str,
        locale_name,
        locale_english_full_name,
        language_english_name,
        locale_local_name,
        acp_str,
        oemcp_str,
        country_abbrev,
        language_abbrev,
    ) = row
    locale_id = int(locale_id_str, 16)
    acp = int(acp_str)
    oemcp = int(oemcp_str)
    return {
        "locale": locale_name,
        "locale_id": locale_id,
        "language": language_english_name,
        "locale_name_english": locale_english_full_name,
        "locale_name_local": locale_local_name,
        "acp": acp,
        "oemcp": oemcp,
        "country_abbrev": country_abbrev,
        "language_abbrev": language_abbrev,
    }


parser = TableMiningParser()
with open("nls_info.html", encoding="UTF-8") as f:
    parser.feed(f.read())
content_list = [convert_row(r) for r in parser.content_list]
with open("nls_info.json", "w", encoding="UTF-8") as f:
    json.dump(content_list, f)
