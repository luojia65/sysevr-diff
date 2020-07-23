static ssize_t local_readlink(FsContext *fs_ctx, V9fsPath *fs_path,
                              char *buf, size_t bufsz)
{
    ssize_t tsize = -1;

    if ((fs_ctx->export_flags & V9FS_SM_MAPPED) ||
        (fs_ctx->export_flags & V9FS_SM_MAPPED_FILE)) {
        int fd;

        fd = local_open_nofollow(fs_ctx, fs_path->data, O_RDONLY, 0);
        if (fd == -1) {
            return -1;
        }
        do {
            tsize = read(fd, (void *)buf, bufsz);
        } while (tsize == -1 && errno == EINTR);
        close_preserve_errno(fd);
    } else if ((fs_ctx->export_flags & V9FS_SM_PASSTHROUGH) ||
               (fs_ctx->export_flags & V9FS_SM_NONE)) {
        char *dirpath = g_path_get_dirname(fs_path->data);
        char *name = g_path_get_basename(fs_path->data);
        int dirfd;

        dirfd = local_opendir_nofollow(fs_ctx, dirpath);
        if (dirfd == -1) {
            goto out;
        }

        tsize = readlinkat(dirfd, name, buf, bufsz);
        close_preserve_errno(dirfd);
    out:
        g_free(name);
        g_free(dirpath);
    }
    return tsize;
}
