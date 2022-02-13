typedef struct CDuration
{
  long secs;
  int nanos;
} CDuration;

typedef struct COptions
{
  const CDuration *timeout;
  const char *software;
} COptions;

int get_xor_mapped_address(const char *stun_address,
                           const char *local_port,
                           COptions options,
                           char **result); // extern "C"
