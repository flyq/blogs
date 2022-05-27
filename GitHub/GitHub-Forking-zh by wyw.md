无论您是想回馈开源社区还是在自己的项目上进行协作，了解如何正确地分叉和生成pull请求都是非常重要的。不幸的是，在最初的学习过程中很容易犯错误或者不知道该做什么。我知道我一开始确实遇到了相当大的问题，而且我发现GitHub上和网上的很多信息都相当零碎和不完整-----这里描述过程的一部分，那里描述另一部分，经常这些挂在不同的地方，等等。

为了为自己和其他人合并这些信息，这个简短的教程是我发现的非常标准的过程，用于创建fork，执行工作，发出pull request并将该拉取请求merge回原始项目中。

## 创建一个 Fork

只需转到GitHub页面，然后单击“ Fork”按钮即可，就这么简单。完成此操作后，可以使用自己喜欢的git客户端来克隆你的repo，或者直接进入命令行：

```
# 下载你的fork到本地
git clone git@github.com:USERNAME/FORKED-PROJECT.git
```



## 保持Fork自动更新

尽管这不是很必要的步骤，但是如果你打算做的事情不只是一个小小的快速安装，还需要通过跟踪fork的原始“上游”存储库来确保保持最新状态。为此，需要添加一个remote：

```
# 添加 '上游' 库到远程列表
git remote add upstream https://github.com/UPSTREAM-USER/ORIGINAL-PROJECT.git

# 验证名为“upstream”的新远程仓库
git remote -v
```

每当你想使用最新的上游更改来更新fork时，首先需要获取上游存储库的分支和最新提交，以将它们带入存储库：

```
# 获取上游远程仓库
git fetch upstream

# 查看所有分支，包括来自上游的分支
git branch -va
```

现在，chekout自己的master分支并合并上游仓库的master分支：

```
# checkout你的master分支和上游合并
git checkout master
git merge upstream/master
```

如果本地master分支上没有唯一的提交，则git只会执行fast-forward（快进合并）。但是，如果您一直在对master进行更改（在大多数情况下，您可能不需要-[请参见下一部分](https://gist.github.com/Chaser324/ce0505fbed06b947d962#doing-your-work)，您可能必须处理冲突。这样做时，请注意尊重上游所做的更改。

现在，你的本地master分支是最新的，上游进行了所有修改。



## 进行你自己的工作

### 创建一个分支

每当您开始研究新功能或解决bug的时候，最重要的是创建一个新分支。它不仅是正确的git工作流，而且还使你的更改井井有条，并与master分支分离，因此你可以轻松地为完成的每个任务提交和管理多个pull requests。

创建一个新分支并开始工作：

```
# Checkout master分支 - 当你希望你的新分支来自master
git checkout master

# 创建一个名为newfeature的新分支(给你的分支一个简单的有意义的名称)
git branch newfeature

# 切换到您的新分支
git checkout newfeature
```

现在，你可以自由自在的码代码并且可以随意的做成修改了



## 提交一个 Pull Request

### 整理你的工作

在提交 pull request之前，您可能需要做一些事情来清理分支，并使原始仓库的维护者尽可能可以简单地测试，接受和合并你的工作。

如果你对上游master分支进行了任何提交，则应重新设置开发分支以便使合并操作是一个简单的快速合并，不需要进行任何冲突解决工作。

```
# Fetch upstream master and merge with your repo's master branch
git fetch upstream
git checkout master
git merge upstream/master

# If there were any new commits, rebase your development branch
git checkout newfeature
git rebase master
```

现在，可能需要将一些较小的提交压缩为少量较大的更紧密的提交。你可以使用交互式复位基底来执行此操作：

```
# 在你的开发分支上重新建立所有提交的基础
git checkout 
git rebase -i master
```

这将打开一个文本编辑器，您可以在其中指定要压缩的内容。

### 提交

将所有更改提交并push到GitHub后，转到GitHub上的fork页面，选择你的开发分支，然后单击pull request按钮。如果您需要对提取请求进行任何调整，只需将更新推送到GitHub。您的拉取请求将自动跟踪开发分支上的更改并更新。



## 接受和合并Pull Request

请注意，不同于前面的部分是从创建派生并生成pull request的人员的角度编写的，而本节是从处理传入的pull request的原始存储库所有者的角度编写的。因此，在“ forker”将原始存储库称为`upstream`的地方，我们现在将其视为该原始存储库和标准`origin` remote的所有者。



### 检验并测试 Pull Requests

打开`.git/config`文件并在`[remote "origin"]`下面添加新行：

```
fetch = +refs/pull/*/head:refs/pull/origin/*
```

现在，您可以获取并检查任何Pull Requests，以便可以对其进行测试：

```
# 获取所有拉取请求分支
git fetch origin

# 根据其编号检查给定的pull request分支
git checkout -b 999 pull/origin/999
```

请记住，这些分支将是只读的，无法进行任何更改。

### 自动合并Pull Requests

如果合并是一个简单的fast-forward，则可以通过单击GitHub上pull request页面上的按钮来自动进行合并。

### 手动合并Pull Request

要手动进行合并，您需要在源存储库中checkout目标分支，直接从fork中提取，然后合并后push。

```
# Checkout要合并到目标仓库中的分支
git checkout master

# 从完成pull request开发的fork仓库中拉出开发分支
git pull https://github.com/forkuser/forkedrepo.git newfeature

# 合并开发分支
git merge newfeature

# push合并好新功能的master分支
git push origin master
```

现在，你已经完成了开发分支的工作，可以随意删除它了。

```
git branch -d newfeature
```

**版权**

Chase Pettit版权所有2017

MIT License，[http：//www.opensource.org/licenses/mit-license.php](http://www.opensource.org/licenses/mit-license.php)

**附加阅读**

- [Atlassian - Merging vs. Rebasing](https://www.atlassian.com/git/tutorials/merging-vs-rebasing)

**资料来源**

- [GitHub - Fork a Repo](https://help.github.com/articles/fork-a-repo)
- [GitHub - Syncing a Fork](https://help.github.com/articles/syncing-a-fork)
- [GitHub - Checking Out a Pull Request](https://help.github.com/articles/checking-out-pull-requests-locally)