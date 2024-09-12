((rust-mode
  (eval . (progn
            (setenv
             "CARGO_HOME"
             (string-join
              (list (locate-dominating-file default-directory "Cargo.toml")
                    ".cargo-env")))
            (setenv
             "PATH"
             (string-join
              (list (getenv "PATH")
                    ":/usr/lib/sdk/rust-stable/bin")))
            (add-to-list 'exec-path "/usr/lib/sdk/rust-stable/bin")))))
