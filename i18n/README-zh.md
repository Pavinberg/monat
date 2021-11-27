# monat -- Modern shell Navigator

## 介绍

`monat` 是一个关注于文件系统导航的 Unix shell 辅助命令，尤其针对大项目设计。想象这样的一个场景，你在一个比较大的项目的项目目录下，许多文件都在较深的目录结构中，你常常需要访问某些文件、重命名或是移动文件、在一些子目录下执行一些命令后再返回项目目录等等。**每次操作都要输长而繁杂的路径是非常繁琐的**。

`monat` 就是被设计成改善这种长路径输入过程的工具。

:warning: 本项目尚处开发阶段，欢迎大家的贡献。

## 示例

![monat-demo](../images/monat-demo.gif)

- 重命名：

	```shell
	$ mv path/to/the/foo.txt path/to/the/bar.txt
	```
	
	在 `monat` 中我们可以输入如下更短的命令：
	
	```shell
	$ mn path/to/the/foo.txt ,/bar.txt
	```
	
- 将文件向上移动一级目录：

	```shell
	$ mv path/to/another/foo.txt path/to/
	```
	
	在 `monat` 中，我们可以输入：
	
	```shell
	$ mn path/to/another/foo.txt ,/../
	```

- 使用历史记录：

	被 `monat` 访问过的目录都会被记录到历史中。当输入 `mn` 命令而不加参数时可以列出历史记录。在 Bash 中，也可以通过按下 `<TAB>` 来做自动补全。

	```shell
	$ mn ,<TAB><TAB>
	1 -- path/to/the
	2 -- path/to/another
	3 -- path/to
	```
	
	随后，你可以在逗号后输入一个数字来选择路径做前缀：
	
	```shell
	$ mn ,1/bar.txt ,2
	```

	这便等价于输入：
	
	```shell
	$ mv path/to/the/bar.txt path/to/another/
	```
	
- 借助 `monat` 执行另一个命令：

	```shell
	$ mn -c vim ,2/bar.txt
	```
	
	相当于：
	
	```shell
	$ vim path/to/another/bar.txt
	```
	
- 进入一个子目录并运行一个命令：

	```shell
	$ cd path/to/the/
	$ make
	$ cd -
	```
	
	尽管你可以写成一行，但如此你便不能对文件名做自动补全：
	
	```shell
	$ cd path/to/the/ && make; cd -
	```
	
	而在 `monat` 中，我们可以输入：
	
	```shell
	$ mn -d ,1 -c make
	```
	
	这样写的优点在于：
	
	1. 用一行写下更短的命令， 
	
	2. `make` 命令可以通过按下 `<TAB>` 进行自动补全。如果你的命令还要传递文件名作为参数，`monat` 也可以直接为你补全文件名。
	
记住在 Bash 中你总是可以按下 `<TAB>` 来补全路径、命令名和文件名，包括逗号与数字表示的路径，这样的输入非常的方便快速。对于其它 shell 的支持我们会在后续添加。

## 安装

### macOS

使用 [Homebrew](https://brew.sh)：

```shell
$ brew tap Pavinberg/monat
$ brew install monat
```

想要使用 Bash 的自动补全，需要向 `~/.bashrc` 中添加一行内容。运行以下命令即可：

```shell
$ echo "source $(brew --prefix)/etc/bash_completion" >> ~/.bashrc
```

这会使用 [*bash-completion*](https://formulae.brew.sh/formula/bash-completion) 来进行补全。重启 Bash 或执行 `source ~/.bashrc` 刷新。

卸载：

```shell
$ brew uninstall monat
```

### Debian/Ubuntu

下载 `deb` 文件后安装：

```shell
$ curl -LO https://github.com/Pavinberg/monat/releases/download/v0.1.1/monat_0.1.1_amd64.deb
$ sudo dpkg -i monat_0.1.1_amd64.deb
```

安卓后即可删除 `monat_0.1.1_amd64.deb` 文件。`bash-completion` 功能在 Ubuntu 中默认开启，如果没有，你需要在 `~/.bashrc` 中加入以下内容：

```shell
# enable programmable completion features (you don't need to enable
# this, if it's already enabled in /etc/bash.bashrc and /etc/profile
# sources /etc/bash.bashrc).
if ! shopt -oq posix; then
   if [ -f /usr/share/bash-completion/bash_completion ]; then
     . /usr/share/bash-completion/bash_completion
   elif [ -f /etc/bash_completion ]; then
     . /etc/bash_completion
   fi
fi
```

卸载：

```shell
$ sudo dpkg -r monat
```

## 使用说明

### 语法

`monat` 的语法十分简单。以逗号（,）开头的词都被成为*路径缩影*。

如果输入了两个路径而第二个路径以逗号开头，那么这个逗号就代表了前一个路径的前缀，就如第一个示例所示。

如果逗号后跟随了数字 `i`，那么就代表着历史记录中的第 `i` 条记录。

### 参数

`monat` 的功能随着*路径缩影*参数的数量变化而变化。

1. 0 个*路径缩影*：列出当前的历史记录

2. 1 个*路径缩影*：`monat` 相当于 `ls`

3. 2 个*路径缩影*：`monat` 相当于 `mv`

### 历史记录文件

`monat` 把历史记录存储到每个项目的 `.monat/history` 文件中。在开始之前，先在项目目录下使用 `mn -l` 命令来设置当前目录为项目根目录。如果没有在当前目录下找到历史文件，就会在用户目录 `~/.monat/history` 下寻找。

历史记录的数量上限为 10 条，并且按照先入先出（First-In-First-Out, FIFO） 的方式来管理。

### 折中

`monat` 使用时必须保证涉及的文件名本身不是逗号开头的。在现实中这种命名方式十分少见，所以我们认为这样的语法可行。

## 贡献

`monat` 还需要更丰富的特色功能。未来的工作：

1. 支持 `ls` 和 `mv` 命令的参数。

2. 现阶段的自动补全仅支持 Bash，希望支持更多的 shell 的自动补全（尤其是 zsh）。
