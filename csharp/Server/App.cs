using System;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.OpenApi.Models;
using Cassandra;
using System.Runtime.CompilerServices;
using CarePet.Model;

namespace CarePet.Server
{
    public class App
    {
        public static void Main(string[] args)
        {
            var config = Config.Parse(new Config(), args);

            var builder = WebApplication.CreateBuilder(args);

            builder.WebHost.ConfigureKestrel(options =>
            {
                options.ListenAnyIP(8000);
            });
            builder.Services.AddSingleton(config);

            builder.Services.AddSingleton<ISession>(sp =>
            {
                var cfg = sp.GetRequiredService<Config>();
                return cfg.Builder(Config.Keyspace).Build().Connect();
            });

            builder.Services.AddSingleton<Mapper>(serviceProvider =>
            {
                var session = serviceProvider.GetRequiredService<ISession>();
                session.ChangeKeyspace(Config.Keyspace);
                return new Mapper(session);
            });

            builder.Services.AddControllers();

            builder.Services.AddEndpointsApiExplorer();
            builder.Services.AddSwaggerGen(c =>
            {
                c.SwaggerDoc("v1", new OpenApiInfo
                {
                    Title = "CarePet",
                    Version = "0.1",
                    Description = "CarePet: An Example IoT Use Case for Hands-On App Developers",
                    License = new OpenApiLicense
                    {
                        Name = "Apache 2.0",
                        Url = new Uri("https://github.com/scylladb/care-pet/blob/master/LICENSE")
                    },
                    Contact = new OpenApiContact
                    {
                        Url = new Uri("https://github.com/scylladb/care-pet")
                    }
                });
                var xmlFile = $"{System.Reflection.Assembly.GetExecutingAssembly().GetName().Name}.xml";
                var xmlPath = System.IO.Path.Combine(AppContext.BaseDirectory, xmlFile);
                if (System.IO.File.Exists(xmlPath))
                {
                    c.IncludeXmlComments(xmlPath);
                }
            });

            var app = builder.Build();

            if (app.Environment.IsDevelopment())
            {
                app.UseSwagger();
                app.UseSwaggerUI(c =>
                {
                    c.SwaggerEndpoint("/swagger/v1/swagger.json", "CarePet v0.1");
                    c.RoutePrefix = string.Empty;
                    c.DocumentTitle = "CarePet API";
                });
            }

            app.UseHttpsRedirection();
            app.UseAuthorization();
            app.MapControllers();
            app.Run();
        }
    }
}
