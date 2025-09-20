use svcs_lexer::Token;
use std::ops::Range;
use std::fmt;

#[derive(Debug)]
pub struct CstNode {
    pub node_type: String, // semantic type: Port, AssignStatement, Instance, etc.
    pub kind: String,      // token kind or node label
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub children: Vec<CstNode>,
}

#[derive(Debug)]
pub struct Cst {
    pub root: CstNode,
}


/// Compute line and column from a byte offset in the input string
fn get_line_col(input: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    let mut count = 0;
    for c in input.chars() {
        if count == offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        count += c.len_utf8();
    }
    (line, col)
}

pub fn parse_tokens_with_spans(input: &str, tokens: &[(Token, Range<usize>)]) -> Result<Cst, String> {
    let mut children = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let (ref tok, ref span) = tokens[i];
        let lexeme = input.get(span.clone()).unwrap_or("").to_string();
        let (line, column) = get_line_col(input, span.start);
        if *tok == Token::Module {
            // Start of module
            let mut module_children = Vec::new();
            // Add the 'module' keyword
            module_children.push(CstNode {
                node_type: "Keyword".to_string(),
                kind: format!("{:?}", tok),
                lexeme: lexeme.clone(),
                line,
                column,
                children: Vec::new(),
            });
            i += 1;
            // Identifier (module name)
            if i < tokens.len() && tokens[i].0 == Token::Identifier {
                let (ref t, ref tspan) = tokens[i];
                let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                let (tline, tcol) = get_line_col(input, tspan.start);
                module_children.push(CstNode {
                    node_type: "ModuleName".to_string(),
                    kind: format!("{:?}", t),
                    lexeme: tlex,
                    line: tline,
                    column: tcol,
                    children: Vec::new(),
                });
                i += 1;
            }
            // Port list (optional)
            if i < tokens.len() && tokens[i].0 == Token::LeftParen {
                let mut port_children = Vec::new();
                // LeftParen
                let (ref t, ref tspan) = tokens[i];
                let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                let (tline, tcol) = get_line_col(input, tspan.start);
                i += 1;
                // Group ports: (input/output/inout ... Identifier ... ,)
                while i < tokens.len() && tokens[i].0 != Token::RightParen {
                    let start = i;
                    if matches!(tokens[i].0, Token::Input | Token::Output | Token::Inout) {
                        let dir_token = &tokens[i];
                        let (dir, dir_span) = dir_token;
                        let dir_lex = input.get(dir_span.clone()).unwrap_or("").to_string();
                        let (dir_line, dir_col) = get_line_col(input, dir_span.start);
                        i += 1;
                        // Identifier
                        if i < tokens.len() && tokens[i].0 == Token::Identifier {
                            let (ref id, ref id_span) = tokens[i];
                            let id_lex = input.get(id_span.clone()).unwrap_or("").to_string();
                            let (id_line, id_col) = get_line_col(input, id_span.start);
                            i += 1;
                            // Optional comma
                            let mut comma_child = None;
                            if i < tokens.len() && tokens[i].0 == Token::Comma {
                                let (ref c, ref cspan) = tokens[i];
                                let c_lex = input.get(cspan.clone()).unwrap_or("").to_string();
                                let (c_line, c_col) = get_line_col(input, cspan.start);
                                comma_child = Some(CstNode {
                                    node_type: "Comma".to_string(),
                                    kind: format!("{:?}", c),
                                    lexeme: c_lex,
                                    line: c_line,
                                    column: c_col,
                                    children: Vec::new(),
                                });
                                i += 1;
                            }
                            let mut port_kids = vec![
                                CstNode {
                                    node_type: "Direction".to_string(),
                                    kind: format!("{:?}", dir),
                                    lexeme: dir_lex,
                                    line: dir_line,
                                    column: dir_col,
                                    children: Vec::new(),
                                },
                                CstNode {
                                    node_type: "PortName".to_string(),
                                    kind: format!("{:?}", id),
                                    lexeme: id_lex,
                                    line: id_line,
                                    column: id_col,
                                    children: Vec::new(),
                                },
                            ];
                            if let Some(comma) = comma_child { port_kids.push(comma); }
                            port_children.push(CstNode {
                                node_type: "Port".to_string(),
                                kind: "Port".to_string(),
                                lexeme: String::new(),
                                line: 0,
                                column: 0,
                                children: port_kids,
                            });
                            continue;
                        }
                    }
                    // Fallback: add as generic token
                    let (ref t, ref tspan) = tokens[i];
                    let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                    let (tline, tcol) = get_line_col(input, tspan.start);
                    port_children.push(CstNode {
                        node_type: "Token".to_string(),
                        kind: format!("{:?}", t),
                        lexeme: tlex,
                        line: tline,
                        column: tcol,
                        children: Vec::new(),
                    });
                    i += 1;
                }
                // RightParen
                if i < tokens.len() && tokens[i].0 == Token::RightParen {
                    let (ref t, ref tspan) = tokens[i];
                    let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                    let (tline, tcol) = get_line_col(input, tspan.start);
                    i += 1;
                    port_children.push(CstNode {
                        node_type: "Paren".to_string(),
                        kind: format!("{:?}", t),
                        lexeme: tlex,
                        line: tline,
                        column: tcol,
                        children: Vec::new(),
                    });
                }
                module_children.push(CstNode {
                    node_type: "PortList".to_string(),
                    kind: "PortList".to_string(),
                    lexeme: String::new(),
                    line: 0,
                    column: 0,
                    children: port_children,
                });
            }
            // Semicolon
            if i < tokens.len() && tokens[i].0 == Token::Semicolon {
                let (ref t, ref tspan) = tokens[i];
                let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                let (tline, tcol) = get_line_col(input, tspan.start);
                module_children.push(CstNode {
                    node_type: "Semicolon".to_string(),
                    kind: format!("{:?}", t),
                    lexeme: tlex,
                    line: tline,
                    column: tcol,
                    children: Vec::new(),
                });
                i += 1;
            }
            // Module body (everything until EndModule)
            let mut body_children = Vec::new();
            while i < tokens.len() && tokens[i].0 != Token::EndModule {
                // Assign statement: AssignKeyword Identifier Assign ... Semicolon
                if i+4 <= tokens.len() && tokens[i].0 == Token::AssignKeyword && tokens[i+1].0 == Token::Identifier && tokens[i+2].0 == Token::Assign {
                    let mut assign_kids = Vec::new();
                    for j in 0..5 { // assign, id, =, expr, ;
                        let (ref t, ref tspan) = tokens[i+j];
                        let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                        let (tline, tcol) = get_line_col(input, tspan.start);
                        assign_kids.push(CstNode {
                            node_type: if j==0 {"AssignKeyword".to_string()} else if j==1 {"LHS".to_string()} else if j==2 {"AssignOp".to_string()} else if j==4 {"Semicolon".to_string()} else {"Expr".to_string()},
                            kind: format!("{:?}", &tokens[i+j].0),
                            lexeme: tlex,
                            line: tline,
                            column: tcol,
                            children: Vec::new(),
                        });
                    }
                    body_children.push(CstNode {
                        node_type: "AssignStatement".to_string(),
                        kind: "AssignStatement".to_string(),
                        lexeme: String::new(),
                        line: 0,
                        column: 0,
                        children: assign_kids,
                    });
                    i += 5;
                    continue;
                }
                // Instance: Identifier Identifier LeftParen ... RightParen Semicolon
                if i+5 <= tokens.len() && tokens[i].0 == Token::Identifier && tokens[i+1].0 == Token::Identifier && tokens[i+2].0 == Token::LeftParen {
                    let mut inst_kids = Vec::new();
                    for j in 0..2 { // module name, instance name
                        let (ref t, ref tspan) = tokens[i+j];
                        let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                        let (tline, tcol) = get_line_col(input, tspan.start);
                        inst_kids.push(CstNode {
                            node_type: if j==0 {"ModuleType".to_string()} else {"InstanceName".to_string()},
                            kind: format!("{:?}", &tokens[i+j].0),
                            lexeme: tlex,
                            line: tline,
                            column: tcol,
                            children: Vec::new(),
                        });
                    }
                    // Port connections (everything inside parens)
                    let mut portcon_kids = Vec::new();
                    let mut k = i+3;
                    while k < tokens.len() && tokens[k].0 != Token::RightParen {
                        let (ref t, ref tspan) = tokens[k];
                        let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                        let (tline, tcol) = get_line_col(input, tspan.start);
                        portcon_kids.push(CstNode {
                            node_type: "PortConnection".to_string(),
                            kind: format!("{:?}", t),
                            lexeme: tlex,
                            line: tline,
                            column: tcol,
                            children: Vec::new(),
                        });
                        k += 1;
                    }
                    // RightParen
                    if k < tokens.len() && tokens[k].0 == Token::RightParen {
                        let (ref t, ref tspan) = tokens[k];
                        let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                        let (tline, tcol) = get_line_col(input, tspan.start);
                        portcon_kids.push(CstNode {
                            node_type: "Paren".to_string(),
                            kind: format!("{:?}", t),
                            lexeme: tlex,
                            line: tline,
                            column: tcol,
                            children: Vec::new(),
                        });
                        k += 1;
                    }
                    // Semicolon
                    if k < tokens.len() && tokens[k].0 == Token::Semicolon {
                        let (ref t, ref tspan) = tokens[k];
                        let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                        let (tline, tcol) = get_line_col(input, tspan.start);
                        portcon_kids.push(CstNode {
                            node_type: "Semicolon".to_string(),
                            kind: format!("{:?}", t),
                            lexeme: tlex,
                            line: tline,
                            column: tcol,
                            children: Vec::new(),
                        });
                        k += 1;
                    }
                    inst_kids.push(CstNode {
                        node_type: "PortConnections".to_string(),
                        kind: "PortConnections".to_string(),
                        lexeme: String::new(),
                        line: 0,
                        column: 0,
                        children: portcon_kids,
                    });
                    body_children.push(CstNode {
                        node_type: "Instance".to_string(),
                        kind: "Instance".to_string(),
                        lexeme: String::new(),
                        line: 0,
                        column: 0,
                        children: inst_kids,
                    });
                    i = k;
                    continue;
                }
                // Fallback: generic token
                let (ref t, ref tspan) = tokens[i];
                let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                let (tline, tcol) = get_line_col(input, tspan.start);
                body_children.push(CstNode {
                    node_type: "Token".to_string(),
                    kind: format!("{:?}", t),
                    lexeme: tlex,
                    line: tline,
                    column: tcol,
                    children: Vec::new(),
                });
                i += 1;
            }
            if !body_children.is_empty() {
                module_children.push(CstNode {
                    node_type: "ModuleBody".to_string(),
                    kind: "ModuleBody".to_string(),
                    lexeme: String::new(),
                    line: 0,
                    column: 0,
                    children: body_children,
                });
            }
            // EndModule
            if i < tokens.len() && tokens[i].0 == Token::EndModule {
                let (ref t, ref tspan) = tokens[i];
                let tlex = input.get(tspan.clone()).unwrap_or("").to_string();
                let (tline, tcol) = get_line_col(input, tspan.start);
                module_children.push(CstNode {
                    node_type: "Keyword".to_string(),
                    kind: format!("{:?}", t),
                    lexeme: tlex,
                    line: tline,
                    column: tcol,
                    children: Vec::new(),
                });
                i += 1;
            }
            // Wrap into a ModuleDeclaration CST node
            children.push(CstNode {
                node_type: "ModuleDeclaration".to_string(),
                kind: "ModuleDeclaration".to_string(),
                lexeme: String::new(),
                line: 0,
                column: 0,
                children: module_children,
            });
        } else {
            // Fallback: tokens outside modules
            children.push(CstNode {
                node_type: "Token".to_string(),
                kind: format!("{:?}", tok),
                lexeme,
                line,
                column,
                children: Vec::new(),
            });
            i += 1;
        }
    }
    Ok(Cst {
        root: CstNode {
            node_type: "SourceFile".to_string(),
            kind: "SourceFile".to_string(),
            lexeme: String::new(),
            line: 0,
            column: 0,
            children,
        },
    })
}


impl fmt::Display for Cst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_node(node: &CstNode, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            for _ in 0..indent {
                write!(f, "  ")?;
            }
            if node.kind == "ModuleDeclaration" || node.kind == "SourceFile" {
                writeln!(f, "{}", node.kind)?;
            } else {
                writeln!(f, "{}  \"{}\"  @{}:{}", node.kind, node.lexeme, node.line, node.column)?;
            }
            for child in &node.children {
                fmt_node(child, f, indent + 1)?;
            }
            Ok(())
        }
        fmt_node(&self.root, f, 0)
    }
}

