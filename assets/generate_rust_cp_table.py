import json
from pathlib import Path
from sys import argv
from dataclasses import dataclass
from typing import Dict, Optional
from collections import deque


assets_dir = Path(argv[0]).parent

with open(assets_dir / "nls_info.json", encoding="UTF-8") as f:
    table = json.load(f)


@dataclass
class CodePagesInfo:
    acp: int
    oemcp: int


@dataclass
class TreeNode:
    codepage: Optional[CodePagesInfo] = None
    sub: Optional[Dict[str, "TreeNode"]] = None


tree: Dict[str, TreeNode] = {}
for locale_info in table:
    acp = locale_info["acp"]
    oemcp = locale_info["oemcp"]
    locale_part = locale_info["locale"]
    if acp <= 1 or oemcp <= 1:
        continue
    dic = tree
    node = None
    for s in locale_part.split("-"):
        if s not in dic:
            dic[s] = TreeNode()
        node = dic[s]
        if node.sub is None:
            node.sub = {}
        dic = node.sub
    if node is not None:
        if node.sub is not None and not node.sub:
            node.sub = None
        node.codepage = CodePagesInfo(acp, oemcp)

stack = deque()
stack.extend(
    (
        ("root", subname, f"map_{subname.lower()}", subnode, True)
        for subname, subnode in reversed(sorted(tree.items()))
    )
)
with open(
    assets_dir.resolve().parent / "src" / "locale_to_cp_map.rs", "w", encoding="UTF-8"
) as f:
    print(
        """use super::cp_table_type;
use ahash::AHashMap;
use cp_table_type::TableNode::*;
use cp_table_type::*;
use lazy_static::lazy_static;

lazy_static! {
    /// Hash tree that retains the correspondence of locale elements to code pages
    pub static ref LOCALE_TO_CP_MAP: AHashMap<&'static str, TableNode> = {""",
        file=f,
    )
    print(f"        let mut root = AHashMap::with_capacity({len(tree)});", file=f)
    while stack:
        parent, locale_part, locale_var, node, first_time = stack.pop()
        locale_part = locale_part.lower()
        if first_time:
            if node.sub is not None:
                n_sub_items = len(node.sub)
                stack.append((parent, locale_part, locale_var, node, False))
                stack.extend(
                    (
                        (
                            locale_var,
                            subname,
                            f"{locale_var}_{subname.lower()}",
                            subnode,
                            True,
                        )
                        for subname, subnode in reversed(sorted(node.sub.items()))
                    )
                )
                print(
                    f"        let mut {locale_var} = AHashMap::with_capacity({n_sub_items});",
                    file=f,
                )
            elif node.codepage is not None:
                print(
                    f"""        {parent}.insert("{locale_part}", WithCP(CodePage::new({node.codepage.acp}, {node.codepage.oemcp}), None));""",
                    file=f,
                )
        elif node.sub is not None:
            if node.codepage is not None:
                print(
                    f"""        {parent}.insert("{locale_part}", WithCP(CodePage::new({node.codepage.acp}, {node.codepage.oemcp}), Some({locale_var})));""",
                    file=f,
                )
            else:
                print(
                    f"""        {parent}.insert("{locale_part}", WithoutCP({locale_var}));""",
                    file=f,
                )
    print(
        """        return root;
    };
}""",
        file=f,
    )

