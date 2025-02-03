# monoxide-ttf

`monoxide-ttf` is a TTF file **writer** intended for uses with `monoxide`. The writer is designed to map intermediate font representations to a valid TTF file output.

The writer does not provide support for _all_ TTF features, especially obsolete or rarely-used ones. For example, it does not support writing any `cmap` tables other than format 4 and 12. Its design mainly revolves around what `monoxide` uses, and features are only added when needed.
